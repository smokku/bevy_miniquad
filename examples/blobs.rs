use bevy::prelude::*;
use bevy_miniquad::{DrawFn, MiniquadPlugin};
use std::sync::Arc;

pub fn main() {
    log::info!("Starting blobs example");

    App::build()
        .add_default_plugins()
        .add_resource::<DrawFn>(Arc::new(Box::new(draw)))
        .add_plugin(MiniquadPlugin)
        .run();
}

fn draw(_app: &mut App) {
    println!("in draw!");
}
