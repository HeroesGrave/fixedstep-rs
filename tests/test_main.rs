
#[macro_use]
extern crate fixedstep;

#[test]
fn test_legacy_macro()
{
    fixedstep_loop!(
        Step(60, true),
        Update => { true },
        Render(delta) => { println!("Render Delta: {:?}", delta) },
    );
}

#[test]
fn test_new_timer()
{
    let mut fixedstep = fixedstep::FixedStep::start(60.0);
    let mut break_timer = fixedstep::FixedStep::start(10.0).unlimit();

    fixedstep.reset();
    loop {
        while fixedstep.update() {
            println!("Tick");
            // Do updating things
        }
        let _delta = fixedstep.render_delta();
        // Do rendering things

        if break_timer.update() {
            break
        }
    }
}
