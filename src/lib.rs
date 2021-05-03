use ::miniquad::{conf, EventHandlerFree, KeyCode, KeyMods, MouseButton, UserData};
use bevy::app::{App, AppBuilder, AppExit, ManualEventReader, Events, Plugin};
use bevy::input::{
    ElementState,
    keyboard::KeyboardInput,
    mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel},
};
use bevy::math::Vec2;
use bevy::window::{
    CursorMoved, WindowCreated, WindowDescriptor, WindowId, WindowMode, WindowResized,
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

#[derive(Default, Debug)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.set_runner(miniquad_runner);
    }
}

impl Window {
    fn new(width: usize, height: usize) -> Self {
        Window {
            width,
            height,
            ..Window::default()
        }
    }
}

pub fn miniquad_runner(mut app: App) {
    log::debug!("Entering miniquad event loop");

    let mut conf = conf::Conf::default();
    {
        if let Some(desc) = app.world.get_resource::<WindowDescriptor>() {
            conf.window_title = desc.title.clone();
            conf.window_width = desc.width as i32;
            conf.window_height = desc.height as i32;
            conf.fullscreen = match desc.mode {
                WindowMode::Windowed => false,
                WindowMode::BorderlessFullscreen | WindowMode::Fullscreen { .. } => true,
            };
        }
    }

    miniquad::start(conf, |ctx| {
        let (width, height) = ctx.screen_size();

        app.world.insert_resource(ctx);
        app.world.insert_resource(Window::new(width as usize, height as usize));

        {
            let mut window_created_events =
                app.world.get_resource_mut::<Events<WindowCreated>>().unwrap();
            window_created_events.send(WindowCreated {
                id: WindowId::primary(),
            });
        }

        //app.init();

        UserData::free(Stage::new(app))
    });
}

struct Stage {
    app: App,
    app_exit_event_reader: ManualEventReader<AppExit>,
}

impl Stage {
    pub fn new(app: App) -> Self {
        let app_exit_event_reader = ManualEventReader::<AppExit>::default();

        Stage {
            app,
            app_exit_event_reader,
        }
    }
}

impl EventHandlerFree for Stage {
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        // println!("key_down_event");
        let mut keyboard_input_events = self
            .app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();
        let input_event = KeyboardInput {
            scan_code: 0,
            state: ElementState::Pressed,
            key_code: convert_virtual_key_code(keycode),
        };
        keyboard_input_events.send(input_event);
    }
    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        // println!("key_up_event");
        let mut keyboard_input_events = self
            .app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();
        let input_event = KeyboardInput {
            scan_code: 0,
            state: ElementState::Released,
            key_code: convert_virtual_key_code(keycode),
        };
        keyboard_input_events.send(input_event);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        // println!("mouse_motion_event {} {}", x, y);
        let mut window = self.app.world.get_resource_mut::<Window>().unwrap();
        window.cursor_x = x as usize;
        window.cursor_y = y as usize;

        let mut cursor_moved_events = self.app.world.get_resource_mut::<Events<CursorMoved>>().unwrap();
        cursor_moved_events.send(CursorMoved {
            id: WindowId::primary(),
            position: Vec2::new(x, y),
        });
    }
    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        // println!("mouse_wheel_event {} {}", x, y);
        let mut mouse_wheel_input_events =
            self.app.world.get_resource_mut::<Events<MouseWheel>>().unwrap();
        mouse_wheel_input_events.send(MouseWheel {
            unit: MouseScrollUnit::Line,
            x,
            y,
        });
    }
    fn mouse_button_down_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        // println!("mouse_button_down_event");
        let mut mouse_button_input_events = self
            .app
            .world
            .get_resource_mut::<Events<MouseButtonInput>>()
            .unwrap();
        mouse_button_input_events.send(MouseButtonInput {
            button: convert_mouse_button(button),
            state: ElementState::Pressed,
        });
    }
    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        // println!("mouse_button_up_event");
        let mut mouse_button_input_events = self
            .app
            .world
            .get_resource_mut::<Events<MouseButtonInput>>()
            .unwrap();
        mouse_button_input_events.send(MouseButtonInput {
            button: convert_mouse_button(button),
            state: ElementState::Released,
        });
    }
    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
        // println!("raw_mouse_motion {} {}", dx, dy);
        let mut mouse_motion_events = self.app.world.get_resource_mut::<Events<MouseMotion>>().unwrap();
        mouse_motion_events.send(MouseMotion {
            delta: Vec2::new(dx, dy),
        });
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        println!("resize_event {} {}", width, height);
        let mut window = self.app.world.get_resource_mut::<Window>().unwrap();
        window.width = width as usize;
        window.height = height as usize;

        let mut window_resized_events = self
            .app
            .world
            .get_resource_mut::<Events<WindowResized>>()
            .unwrap();
        window_resized_events.send(WindowResized {
            id: WindowId::primary(),
            width,
            height,
        });
    }

    fn update(&mut self) {
        // println!("update");
        if let Some(app_exit_events) = self.app.world.get_resource_mut::<Events<AppExit>>() {
            if self
                .app_exit_event_reader
                .iter(&app_exit_events)
                .next_back()
                .is_some()
            {
                let ctx = self.app.world.get_resource_mut::<::miniquad::Context>().unwrap();
                ctx.request_quit();
            }
        }

        self.app.update();
    }

    fn draw(&mut self) {
        // println!("draw");
        let draw_function = {
            let fn_ref = self
                .app
                .world
                .get_resource::<DrawFn>()
                .expect("Cannot find draw function resource");
            (*fn_ref).clone()
        };

        draw_function(&mut self.app);
    }
}
