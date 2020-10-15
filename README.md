# Bevy engine + miniquad renderer

This is a plugin for [Bevy engine][1] that replaces default windowing and rendering plugins
with [miniquad][2] based one.

[1]: https://github.com/bevyengine/bevy
[2]: https://github.com/not-fl3/miniquad

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "*", default-features = false, path = "../bevy" }
bevy_miniquad = { git = "https://github.com/smokku/bevy_miniquad.git" }
```

You need to implement your own `render` function and add it as a resource:

```rust
App::build()
    .add_default_plugins()
    .add_resource::<DrawFn>(Arc::new(Box::new(draw)))
    .add_plugin(MiniquadPlugin)
```

This plugin exposes `Window` resource with window dimensions and cursor position.

## features

### `log-impl`

This plugin exposes `log` module with API compatible with `log` crate, which
works under every `miniquad` supported platform. See `blobs` example.
