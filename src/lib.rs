use ::miniquad::{conf, EventHandlerFree, KeyCode, KeyMods, MouseButton, UserData};
use bevy_app::{App, AppBuilder, AppExit, EventReader, Events, Plugin};
use bevy_input::{
    keyboard::{ElementState, KeyboardInput},
    mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel},
};
use bevy_math::Vec2;
use bevy_window::{CursorMoved, WindowId};
use std::sync::Arc;

pub use ::miniquad::Context;
pub mod miniquad {
    pub use miniquad::*;
}

#[cfg(feature = "log-impl")]
mod log {
    pub use miniquad::{debug, error, info, log, trace, warn};
}

mod converters;
use converters::*;

pub type DrawFn = Arc<Box<dyn Fn(&mut App) -> () + Send + Sync>>;

#[derive(Default)]
pub struct MiniquadPlugin;

impl Plugin for MiniquadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.set_runner(miniquad_runner);
    }
}

pub fn miniquad_runner(mut app: App) {
    println!("before start");
    log::debug!("Entering miniquad event loop");

    miniquad::start(conf::Conf::default(), |ctx| {
        println!("start");
        app.resources.insert(ctx);
        println!("initialize");
        app.initialize();
        println!("run");
        UserData::free(Stage::new(app))
    });
}

struct Stage {
    app: App,
    app_exit_event_reader: EventReader<AppExit>,
}

impl Stage {
    pub fn new(app: App) -> Self {
        let app_exit_event_reader = EventReader::<AppExit>::default();

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
            .resources
            .get_mut::<Events<KeyboardInput>>()
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
            .resources
            .get_mut::<Events<KeyboardInput>>()
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
        let mut cursor_moved_events = self.app.resources.get_mut::<Events<CursorMoved>>().unwrap();
        cursor_moved_events.send(CursorMoved {
            id: WindowId::primary(),
            position: Vec2::new(x, y),
        });
    }
    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        // println!("mouse_wheel_event {} {}", x, y);
        let mut mouse_wheel_input_events =
            self.app.resources.get_mut::<Events<MouseWheel>>().unwrap();
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
            .resources
            .get_mut::<Events<MouseButtonInput>>()
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
            .resources
            .get_mut::<Events<MouseButtonInput>>()
            .unwrap();
        mouse_button_input_events.send(MouseButtonInput {
            button: convert_mouse_button(button),
            state: ElementState::Released,
        });
    }
    fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
        // println!("raw_mouse_motion {} {}", dx, dy);
        let mut mouse_motion_events = self.app.resources.get_mut::<Events<MouseMotion>>().unwrap();
        mouse_motion_events.send(MouseMotion {
            delta: Vec2::new(dx, dy),
        });
    }

    fn update(&mut self) {
        println!("update");
        if let Some(app_exit_events) = self.app.resources.get_mut::<Events<AppExit>>() {
            if self
                .app_exit_event_reader
                .latest(&app_exit_events)
                .is_some()
            {
                let ctx = self.app.resources.get_mut::<::miniquad::Context>().unwrap();
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
                .resources
                .get::<DrawFn>()
                .expect("Cannot find draw function resource");
            (*fn_ref).clone()
        };

        draw_function(&mut self.app);
    }
}
