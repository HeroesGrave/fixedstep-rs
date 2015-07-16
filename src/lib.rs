
extern crate clock_ticks;

pub fn nano_time() -> u64
{
    clock_ticks::precise_time_ns()
}

#[macro_export]
macro_rules! fixedstep_loop {
    {
        Step($ticks:expr, $skip:expr),
        Update => $Update:block,
        Render($delta:pat) => $Render:block,
    } => {
        {
            let nanos: f64 = 1_000_000_000f64;
            let update_interval: f64 = nanos / ($ticks as f64);
            let skip_threshold: i32 = 3;

            let mut last = $crate::nano_time() as f64;
            let mut accumulator = 0.0f64;

            let mut should_close = false;

            while !should_close
            {
                let now = $crate::nano_time() as f64;
                accumulator += now - last;
                last = now;

                let mut update_count = 0;
                while accumulator > update_interval && update_count < skip_threshold
                {
                    should_close = $Update;
                    update_count += 1;
                    accumulator -= update_interval;
                }

                // Frame skip
                // Do not use for simulations
                if $skip && accumulator > update_interval
                {
                    accumulator = 0.0;
                }

                let $delta = (($crate::nano_time() as f64 - last) / update_interval).min(1.0);
                $Render
            }
        }
    };
    {
        Step($ticks:expr),
        Update => $Update:block,
        Render($delta:pat) => $Render:block,
    } => {
        fixedstep_loop!(
            Step($ticks, true),
            Update => $Update,
            Render($delta) => $Render,
        )
    };
    {
        Update => $Update:block,
        Render($delta:pat) => $Render:block,
    } => {
        fixedstep_loop!(
            Step(60),
            Update => $Update,
            Render($delta) => $Render,
        )
    };
}
