use miniquad::{conf, window, EventHandler, KeyCode, KeyMods, MouseButton};
use bevy_app::{App, AppExit, Plugin};
use bevy_ecs::{event::{ManualEventReader, Events}, entity::Entity, prelude::Resource};
use bevy_input::{
    ButtonState,
    keyboard::{KeyboardInput, NativeKey, Key},
    mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel},
};
use bevy_math::Vec2;
use bevy_window::{
    CursorMoved, WindowCreated, WindowMode, WindowResized, WindowResolution, WindowPlugin, Window as WindowComponent
};
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
                    WindowMode::BorderlessFullscreen |
                    WindowMode::SizedFullscreen |
                    WindowMode::Fullscreen => true,
                };
            }
        }
    }

    miniquad::start(conf, || {
        let ctx: Box<Context> = window::new_rendering_backend();
        app.insert_non_send_resource(MiniquadContext(ctx));

        let (width, height) = window::screen_size();
        let scale = window::dpi_scale();

        let window = window_copy.unwrap_or_else(|| {
            WindowComponent {
                resolution: WindowResolution::new(width as f32, height as f32)
                    .with_scale_factor_override(scale),
                ..Default::default()
            }
        });
        let entity = app.world_mut().spawn(window).id();
        app.world_mut().insert_resource(Window::new(width, height));

        {
            let mut window_created_events =
                app.world_mut().get_resource_mut::<Events<WindowCreated>>().unwrap();
            window_created_events.send(WindowCreated {
                window: entity,
            });
        }

        app.finish();

        Box::new(Stage::new(app, entity))
    });

    AppExit::Success
}

struct Stage {
    app: App,
    app_exit_event_reader: ManualEventReader<AppExit>,
    window_entity: Entity,
}

impl Stage {
    pub fn new(app: App, window_entity: Entity) -> Self {
        let app_exit_event_reader = ManualEventReader::<AppExit>::default();

        Stage {
            app,
            window_entity,
            app_exit_event_reader,
        }
    }
}

impl EventHandler for Stage {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        // println!("key_down_event");
        let mut keyboard_input_events = self
            .app
            .world_mut()
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();
        let input_event = KeyboardInput {
            logical_key: Key::Unidentified(NativeKey::Unidentified),
            window: self.window_entity,
            state: ButtonState::Pressed,
            key_code: convert_virtual_key_code(keycode).unwrap(),
        };
        keyboard_input_events.send(input_event);
    }
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        // println!("key_up_event");
        let mut keyboard_input_events = self
            .app
            .world_mut()
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();
        let input_event = KeyboardInput {
            logical_key: Key::Unidentified(NativeKey::Unidentified),
            window: self.window_entity,
            state: ButtonState::Released,
            key_code: convert_virtual_key_code(keycode).unwrap(),
        };
        keyboard_input_events.send(input_event);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        // println!("mouse_motion_event {} {}", x, y);
        let mut window = self.app.world_mut().get_resource_mut::<Window>().unwrap();
        let delta_x = x - window.cursor_x;
        let delta_y = y - window.cursor_y;
        window.cursor_x = x;
        window.cursor_y = y;

        let mut cursor_moved_events = self.app.world_mut().get_resource_mut::<Events<CursorMoved>>().unwrap();
        cursor_moved_events.send(CursorMoved {
            window: self.window_entity,
            position: Vec2::new(x, y),
            delta: Some(Vec2::new(delta_x, delta_y)),
        });
    }
    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        // println!("mouse_wheel_event {} {}", x, y);
        let mut mouse_wheel_input_events =
            self.app.world_mut().get_resource_mut::<Events<MouseWheel>>().unwrap();
        mouse_wheel_input_events.send(MouseWheel {
            window: self.window_entity,
            unit: MouseScrollUnit::Line,
            x,
            y,
        });
    }
    fn mouse_button_down_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        // println!("mouse_button_down_event");
        let mut mouse_button_input_events = self
            .app
            .world_mut()
            .get_resource_mut::<Events<MouseButtonInput>>()
            .unwrap();
        mouse_button_input_events.send(MouseButtonInput {
            window: self.window_entity,
            button: convert_mouse_button(button),
            state: ButtonState::Pressed,
        });
    }
    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        // println!("mouse_button_up_event");
        let mut mouse_button_input_events = self
            .app
            .world_mut()
            .get_resource_mut::<Events<MouseButtonInput>>()
            .unwrap();
        mouse_button_input_events.send(MouseButtonInput {
            window: self.window_entity,
            button: convert_mouse_button(button),
            state: ButtonState::Released,
        });
    }
    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
        // println!("raw_mouse_motion {} {}", dx, dy);
        let mut mouse_motion_events = self.app.world_mut().get_resource_mut::<Events<MouseMotion>>().unwrap();
        mouse_motion_events.send(MouseMotion {
            delta: Vec2::new(dx, dy),
        });
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        println!("resize_event {} {}", width, height);
        let mut window = self.app.world_mut().get_resource_mut::<Window>().unwrap();
        window.width = width;
        window.height = height;

        let mut window_resized_events = self
            .app
            .world_mut()
            .get_resource_mut::<Events<WindowResized>>()
            .unwrap();
        window_resized_events.send(WindowResized {
            window: self.window_entity,
            width: width,
            height: height,
        });
    }

    fn update(&mut self) {
        // println!("update");
        if let Some(app_exit_events) = self.app.world_mut().get_resource_mut::<Events<AppExit>>() {
            if self
                .app_exit_event_reader
                .read(&app_exit_events)
                .next()
                .is_some()
            {
                window::request_quit();
            }
        }

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
