#[macro_export]
macro_rules! fixedstep_loop {
    {
        Step($ticks:expr, $skip:expr),
        Update => $Update:block,
        Render($delta:pat) => $Render:block,
    } => {
        {
            use std::time::{Duration, Instant};
            let ticks = 1.0 / $ticks as f64;
            let ticks_s = ticks as u64;
            let ticks_ns = (ticks.fract() * 1000_000_000.0) as u32;
            let update_interval = Duration::new(ticks_s, ticks_ns);
            let skip_threshold: i32 = 3;

            let mut last = Instant::now();
            let mut accumulator = Duration::new(0, 0);

            let mut should_close = false;

            while !should_close
            {
                let now = Instant::now();
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
                    accumulator = Duration::new(0, 0);
                }

                let elapsed = last.elapsed();
                let $delta = ((elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1000_000_000.0) / ticks).min(1.0);
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
