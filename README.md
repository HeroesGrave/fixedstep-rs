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
