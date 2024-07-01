// This example is adapted from https://github.com/not-fl3/miniquad/blob/master/examples/blobs.rs

use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
    window::CursorMoved,
};
use bevy_miniquad::{miniquad as mq, DrawFnHandle, MiniquadPlugin, Window, MiniquadContext};
use std::sync::Arc;

pub fn main() {
    log::info!("Starting blobs example");

    App::new()
        .add_plugins(DefaultPlugins)
        // plugin stuff
        .insert_resource(DrawFnHandle(Arc::new(Box::new(draw))))
        .add_plugins(MiniquadPlugin)
        // example stuff
        .add_systems(Startup, configure_stage)
        .add_systems(Update, update)
        .add_systems(Update, mouse_handler)
        .run();
}

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

fn configure_stage(world: &mut World) {
    let renderer = {
        let mut ctx = world.get_non_send_resource_mut::<MiniquadContext>().unwrap();
        let ctx = &mut *(ctx.0);

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = ctx.new_buffer(
            mq::BufferType::VertexBuffer,
            mq::BufferUsage::Immutable,
            mq::BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            mq::BufferType::IndexBuffer,
            mq::BufferUsage::Immutable,
            mq::BufferSource::slice(&indices),
        );

        let bindings = mq::Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx.new_shader(
            mq::ShaderSource::Glsl {
                vertex: shader::VERTEX,
                fragment: shader::FRAGMENT,
            },
            shader::meta(),
        ).unwrap();

        let pipeline = ctx.new_pipeline(
            &[mq::BufferLayout::default()],
            &[
                mq::VertexAttribute::new("pos", mq::VertexFormat::Float2),
                mq::VertexAttribute::new("uv", mq::VertexFormat::Float2),
            ],
            shader,
            mq::PipelineParams::default(),
        );

        let uniforms = shader::Uniforms {
            time: 0.,
            blobs_count: 1,
            blobs_positions: [(0., 0.); 32],
        };

        Renderer {
            pipeline,
            bindings,
            uniforms,
            blobs_velocities: [(0., 0.); 32],
        }
    };

    world.insert_resource(renderer);
}

#[derive(Resource)]
struct Renderer {
    pipeline: mq::Pipeline,
    bindings: mq::Bindings,
    uniforms: shader::Uniforms,
    blobs_velocities: [(f32, f32); 32],
}

fn draw(app: &mut App) {
    let world = app.world_mut();

    world.resource_scope(|world, mut renderer: Mut<Renderer>| {
        let time = world.get_resource::<Time>().unwrap();
        renderer.uniforms.time = time.elapsed().as_secs_f32();

        let ctx = &mut world.get_non_send_resource_mut::<MiniquadContext>().unwrap().0;

        ctx.begin_default_pass(Default::default());
        ctx.apply_pipeline(&renderer.pipeline);
        ctx.apply_bindings(&renderer.bindings);
        ctx.apply_uniforms(mq::UniformsSource::table(&renderer.uniforms));
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
    });
}

fn update(time: Res<Time>, mut renderer: ResMut<Renderer>) {
    for i in 1..renderer.uniforms.blobs_count as usize {
        renderer.uniforms.blobs_positions[i].0 +=
            renderer.blobs_velocities[i].0 * time.delta_seconds() * 0.1;
        renderer.uniforms.blobs_positions[i].1 +=
            renderer.blobs_velocities[i].1 * time.delta_seconds() * 0.1;

        if renderer.uniforms.blobs_positions[i].0 < 0.
            || renderer.uniforms.blobs_positions[i].0 > 1.
        {
            renderer.blobs_velocities[i].0 *= -1.;
        }
        if renderer.uniforms.blobs_positions[i].1 < 0.
            || renderer.uniforms.blobs_positions[i].1 > 1.
        {
            renderer.blobs_velocities[i].1 *= -1.;
        }
    }
}

fn mouse_handler(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut renderer: ResMut<Renderer>,
    window: Res<Window>,
) {
    let w = window.width as f32;
    let h = window.height as f32;

    for event in mouse_button_input_events.read() {
        if event.state == ButtonState::Pressed {
            if renderer.uniforms.blobs_count >= 32 {
                return;
            }

            let x = window.cursor_x as f32;
            let y = window.cursor_y as f32;

            let (x, y) = (x / w, 1. - y / h);
            let (dx, dy) = (quad_rand::gen_range(-1., 1.), quad_rand::gen_range(-1., 1.));

            let renderer = &mut *renderer;
            renderer.uniforms.blobs_positions[renderer.uniforms.blobs_count as usize] = (x, y);
            renderer.blobs_velocities[renderer.uniforms.blobs_count as usize] = (dx, dy);
            renderer.uniforms.blobs_count += 1;
        }
    }

    for event in cursor_moved_events.read() {
        let x = event.position.x;
        let y = event.position.y;

        let (x, y) = (x / w, 1. - y / h);
        renderer.uniforms.blobs_positions[0] = (x, y);
    }
}

// based on: https://www.shadertoy.com/view/XsS3DV
mod shader {
    use bevy_miniquad::miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    uniform vec2 offset;
    varying highp vec2 texcoord;
    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    precision highp float;
    varying vec2 texcoord;
    uniform float time;
    uniform int blobs_count;
    uniform vec2 blobs_positions[32];
    float k = 20.0;
    float field = 0.0;
    vec2 coord;

    void circle ( float r , vec3 col , vec2 offset) {
        vec2 pos = coord.xy;
        vec2 c = offset;
        float d = distance ( pos , c );
        field += ( k * r ) / ( d*d );
    }

    vec3 band ( float shade, float low, float high, vec3 col1, vec3 col2 ) {
        if ( (shade >= low) && (shade <= high) ) {
            float delta = (shade - low) / (high - low);
            vec3 colDiff = col2 - col1;
            return col1 + (delta * colDiff);
        }
        else
            return vec3(0.0,0.0,0.0);
    }

    vec3 gradient ( float shade ) {
        vec3 colour = vec3( (sin(time/2.0)*0.25)+0.25,0.0,(cos(time/2.0)*0.25)+0.25);

        vec3 col1 = vec3(0.01, 0.0, 1.0-0.01);
        vec3 col2 = vec3(1.0-0.01, 0.0, 0.01);
        vec3 col3 = vec3(0.02, 1.0-0.02, 0.02);
        vec3 col4 = vec3((0.01+0.02)/2.0, (0.01+0.02)/2.0, 1.0 - (0.01+0.02)/2.0);
        vec3 col5 = vec3(0.02, 0.02, 0.02);

        colour += band ( shade, 0.0, 0.3, colour, col1 );
        colour += band ( shade, 0.3, 0.6, col1, col2 );
        colour += band ( shade, 0.6, 0.8, col2, col3 );
        colour += band ( shade, 0.8, 0.9, col3, col4 );
        colour += band ( shade, 0.9, 1.0, col4, col5 );

        return colour;
    }

    void main() {
        coord = texcoord;

        for (int i = 0; i < 32; i++) {
            if (i >= blobs_count) { break; } // workaround for webgl error: Loop index cannot be compared with non-constant expression
            circle(.03 , vec3(0.7 ,0.2, 0.8), blobs_positions[i]);
        }

        float shade = min ( 1.0, max ( field/256.0, 0.0 ) );

        gl_FragColor = vec4( gradient(shade), 1.0 );
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("blobs_count", UniformType::Int1),
                    UniformDesc::new("blobs_positions", UniformType::Float2).array(32),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub time: f32,
        pub blobs_count: i32,
        pub blobs_positions: [(f32, f32); 32],
    }
}
