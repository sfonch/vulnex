# vulnex

## Getting Started

To run your game, you need an `App` instance.

Create one like this:
```rust
App::new()
```

### Window Title

You can set the window title using `.title()`:
```rust
App::new()
    .title("My Cool Game".to_string())
```

### Running

Call `.run()` with two closures — one for initialization, one for the game loop:
```rust
App::new()
    .run(|app| {
        // Called once at startup
    }, |app| {
        // Called every frame
    });
```