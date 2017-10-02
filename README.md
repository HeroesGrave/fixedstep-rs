fixedstep-rs
============
A simple library to create a fixed timestep loop in your game (or whatever).

## Usage

A basic loop running at 60Hz:
```rust
fn main()
{
    let mut fixedstep = fixedstep::FixedStep::start(60.0); // 60.0Hz
    while isRunning() {
        while fixedstep.update() {
            // Do updating things
        }
        let _delta = fixedstep.render_delta();
        // Do rendering things
    }
}
```
The rate of updates is 'fixed' using the update frequency specified.
Be sure to note that the outer loop above runs as fast as possible, which means the rendering framerate is uncapped.
You are responsible for implementing the behaviour to cap the framerate (for example: using VSync). Or you could choose not to.

### Frame skipping

By default, `update()` will only return true up to 3 times between calls to `render_delta()`. If you want to disable this, you can use the `unlimit()` method:
```rust
let mut fixedstep = fixedstep::FixedStep::start(60.0).unlimit();
```
However, this is not recommended. If your update functionality takes longer than the update interval, you may get stuck in a loop and be unable to render (of course, if this occurs it means the machine running the program is unable to run at the requested update rate in the first place. In practice, skipping frames prevents small lag spikes from causing multiple frames of lag as the program tries to catch up).

If you want to change the update limit (for example, if you're updating at a higher frequency than you intend on rendering at), you can call the `limit()` method:
```rust
let mut fixedstep = fixedstep::FixedStep::start(60.0).limit(5);
```

Note that the following two lines are equivalent (because the limit defaults to 3):
```rust
let mut fixedstep = fixedstep::FixedStep::start(60.0);
let mut fixedstep = fixedstep::FixedStep::start(60.0).limit(3);
```

### Resetting the loop

Sometimes you may find that you need to create the FixedStep object some time before you begin using it. Time is recorded from the creation of the object, so if you wait 5 seconds before starting the loop, the first update loop will try and run for 5 seconds' worth of updates. To prevent this, you can reset the loop just before you begin using it:

```rust
let mut fixedstep = fixedstep::FixedStep::start(60.0);
// Initialise some other stuff, possibly taking a long time.
// ...
fixedstep.reset();
while isRunning() {
    // ...
}
```
In practice you should never need to do this (because you can move the initialisation right before the loop starts), but if there's some niche case that needs it, the functionality is there.

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
