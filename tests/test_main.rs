
#[macro_use]
extern crate fixedstep;

#[test]
fn test()
{
    fixedstep_loop!(
        Step(60, true),
        Update => { true },
        Render(delta) => { println!("Render Delta: {:?}", delta) },
    );
}
