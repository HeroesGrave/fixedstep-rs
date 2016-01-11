fixedstep-rs
============
A simple macro to create a fixed timestep loop in your game (or whatever).

## Usage

### Simplest:
```rust
fn main() {
    // initialise everything

    fixedstep_loop! {
        Update => {
            // update stuff
            should_terminate() // Return a boolean telling the loop whether to terminate or not.
        },
        Render(delta) => { /* render stuff */ }, // Or Render(_) if you don't need the delta
    }

    // cleanup everything
}
```

### Specify update frequency [Hz]
```rust
fixedstep_loop! {
    Step(60), // 60 is default
    Update => { /* ... */ },
    Render(delta) => { /* ... */ },
}
```
The rate of updates is 'fixed' using the update frequency specified.
The framerate cap has to be controlled in the render block (eg: use VSync).

### Disable frame skipping
```rust
fixedstep_loop! {
    Step(60, false),
    Update => { /* ... */ },
    Render(delta) => { /* ... */ },
}
```
You should only do this if you're running a simulation. With frame-skipping disabled, the framerate may die trying to catch up after a temporary performance hit.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
