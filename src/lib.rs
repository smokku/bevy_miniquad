use bevy_app::{App, AppExit, Plugin, Update};
use bevy_ecs::{entity::Entity, event::EventReader, prelude::Resource};
use bevy_input::{
    keyboard::{Key, KeyboardInput},
    mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel},
    ButtonState,
};
use bevy_math::Vec2;
use bevy_window::{
    CursorMoved, Window as WindowComponent, WindowMode, WindowPlugin, WindowResized,
    WindowResolution,
};
use miniquad::{conf, window, EventHandler, KeyCode, KeyMods, MouseButton};
use std::sync::Arc;

pub use ::miniquad::Context;
pub mod miniquad {
    pub use miniquad::*;
}

#[cfg(feature = "log-impl")]
pub mod log {
    pub use miniquad::{debug, error, info, log, trace, warn};
}

mod converters;
use converters::*;

pub type DrawFn = Arc<Box<dyn Fn(&mut App) -> () + Send + Sync>>;

pub struct DrawFnHandle(pub DrawFn);

impl Resource for DrawFnHandle {}

pub struct MiniquadContext(pub Box<Context>);

#[derive(Default, Debug, Resource)]
pub struct Window {
    pub width: f32,
    pub height: f32,
    pub cursor_x: f32,
    pub cursor_y: f32,
}

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(miniquad_runner);
        app.add_plugins(WindowPlugin::default());
        app.add_systems(Update, app_exit_system);
    }
}

fn app_exit_system(mut event_reader: EventReader<AppExit>) {
    for _app_exit_event in event_reader.read() {
        window::request_quit();
    }
}

impl Window {
    fn new(width: f32, height: f32) -> Self {
        Window {
            width,
            height,
            ..Window::default()
        }
    }
}

pub fn miniquad_runner(mut app: App) -> AppExit {
    log::debug!("Entering miniquad event loop");

    let mut conf = conf::Conf::default();
    let mut window_copy = None;
    {
        let window_settings = app.get_added_plugins::<WindowPlugin>();
        for settings in &window_settings {
            if let Some(ref window) = settings.primary_window {
                window_copy = Some(window.clone());
                conf.window_title = window.title.clone();
                conf.window_width = window.resolution.width() as i32;
                conf.window_height = window.resolution.height() as i32;
                conf.fullscreen = match window.mode {
                    WindowMode::Windowed => false,
                    WindowMode::BorderlessFullscreen(_)
                    | WindowMode::SizedFullscreen(_)
                    | WindowMode::Fullscreen(_) => true,
                };
            }
        }
    }

    miniquad::start(conf, || {
        let ctx: Box<Context> = window::new_rendering_backend();
        app.insert_non_send_resource(MiniquadContext(ctx));

        let (width, height) = window::screen_size();
        let scale = window::dpi_scale();

        let window = window_copy.unwrap_or_else(|| WindowComponent {
            resolution: WindowResolution::new(width as f32, height as f32)
                .with_scale_factor_override(scale),
            ..Default::default()
        });
        let entity = app.world_mut().spawn(window).id();
        app.world_mut().insert_resource(Window::new(width, height));

        app.finish();

        Box::new(Stage::new(app, entity))
    });

    AppExit::Success
}

struct Stage {
    app: App,
    window_entity: Entity,
    last_printable_char: Option<char>,
    last_key_code: Option<KeyCode>,
}

impl Stage {
    pub fn new(app: App, window_entity: Entity) -> Self {
        Stage {
            app,
            window_entity,
            last_printable_char: None,
            last_key_code: None,
        }
    }
}

impl EventHandler for Stage {
    fn char_event(&mut self, character: char, _keymods: KeyMods, repeat: bool) {
        // println!("char_event");
        let input_event = KeyboardInput {
            logical_key: Key::Character(character.to_string().into()),
            window: self.window_entity,
            state: ButtonState::Pressed,
            key_code: convert_virtual_key_code(self.last_key_code.unwrap()).unwrap(),
            repeat,
        };
        //println!("{:?}", input_event);
        self.app.world_mut().send_event(input_event);
        self.last_printable_char = Some(character);
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        // println!("key_down_event");
        self.last_key_code = Some(keycode);
        let key_code = convert_virtual_key_code(keycode).unwrap();
        if key_code_is_printable(key_code) {
            // Wait for the next char event instead.
            return;
        }

        let input_event = KeyboardInput {
            logical_key: key_code_to_unprintable_logical_key(key_code).unwrap(),
            window: self.window_entity,
            state: ButtonState::Pressed,
            key_code,
            repeat,
        };
        //println!("{:?}", input_event);
        self.app.world_mut().send_event(input_event);
    }
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        // println!("key_up_event");
        let key_code = convert_virtual_key_code(keycode).unwrap();
        let logical_key = if key_code_is_printable(key_code) {
            Key::Character(self.last_printable_char.unwrap().to_string().into())
        } else {
            key_code_to_unprintable_logical_key(key_code).unwrap()
        };
        let input_event = KeyboardInput {
            logical_key,
            window: self.window_entity,
            state: ButtonState::Released,
            key_code,
            repeat: false,
        };
        //println!("{:?}", input_event);
        self.app.world_mut().send_event(input_event);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        // println!("mouse_motion_event {} {}", x, y);
        let mut window = self.app.world_mut().get_resource_mut::<Window>().unwrap();
        let delta_x = x - window.cursor_x;
        let delta_y = y - window.cursor_y;
        window.cursor_x = x;
        window.cursor_y = y;

        self.app.world_mut().send_event(CursorMoved {
            window: self.window_entity,
            position: Vec2::new(x, y),
            delta: Some(Vec2::new(delta_x, delta_y)),
        });
    }
    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        // println!("mouse_wheel_event {} {}", x, y);
        self.app.world_mut().send_event(MouseWheel {
            window: self.window_entity,
            unit: MouseScrollUnit::Line,
            x,
            y,
        });
    }
    fn mouse_button_down_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        // println!("mouse_button_down_event");
        self.app.world_mut().send_event(MouseButtonInput {
            window: self.window_entity,
            button: convert_mouse_button(button),
            state: ButtonState::Pressed,
        });
    }
    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        // println!("mouse_button_up_event");
        self.app.world_mut().send_event(MouseButtonInput {
            window: self.window_entity,
            button: convert_mouse_button(button),
            state: ButtonState::Released,
        });
    }
    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
        // println!("raw_mouse_motion {} {}", dx, dy);
        self.app.world_mut().send_event(MouseMotion {
            delta: Vec2::new(dx, dy),
        });
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        println!("resize_event {} {}", width, height);
        let mut window = self.app.world_mut().get_resource_mut::<Window>().unwrap();
        window.width = width;
        window.height = height;

        self.app.world_mut().send_event(WindowResized {
            window: self.window_entity,
            width: width,
            height: height,
        });
    }

    fn update(&mut self) {
        self.app.update();
    }

    fn draw(&mut self) {
        // println!("draw");
        let draw_function = {
            let fn_ref = self
                .app
                .world()
                .get_resource::<DrawFnHandle>()
                .expect("Cannot find draw function resource");
            (*fn_ref).0.clone()
        };

        draw_function(&mut self.app);
    }
}
