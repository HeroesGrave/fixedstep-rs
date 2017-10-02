
use std::time::{Duration, Instant};

const NANOS_PER_SECOND: f64 = 1_000_000_000.0;

pub struct FixedStep {
    last_time: Instant,
    update_interval: Duration,
    accumulator: Duration,
    update_counter: u32,
    update_limit: u32,
}

impl FixedStep {
    /// Create and start a new fixedstep timer with the given frequency in Hz
    pub fn start(hz: f64) -> Self {
        let seconds = 1.0 / hz;
        let full_seconds = seconds as u64;
        let remaining_nanos = (seconds.fract() * NANOS_PER_SECOND) as u32;
        FixedStep {
            update_interval: Duration::new(full_seconds, remaining_nanos),
            last_time: Instant::now(),
            accumulator: Duration::new(0, 0),
            update_counter: 0,
            update_limit: 3,
        }
    }

    /// Set the limit for how many updates can be performed between rendering.
    /// ie: the maximum number of times update() will return true between calls to render_delta
    ///
    /// Use this if rendering on time is more important than keeping the simulation on time
    /// (which is usually the case for video games).
    pub fn limit(mut self, limit: u32) -> Self {
        self.update_limit = limit;
        self
    }

    /// Remove the update limit
    pub fn unlimit(mut self) -> Self {
        self.update_limit = ::std::u32::MAX;
        self
    }

    /// Restarts the timer at the current time and clears any waiting updates.
    pub fn reset(&mut self) {
        self.last_time = Instant::now();
        self.update_counter = 0;
        self.accumulator = Duration::new(0, 0);
    }

    /// Returns true if enough time has elapsed to perform another update.
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        self.accumulator += now - self.last_time;
        self.last_time = now;
        if self.accumulator >= self.update_interval {
            // Time for another update
            self.update_counter += 1;
            if self.update_counter > self.update_limit {
                // If too many updates have occured since the last render,
                // skip any waiting updates and return false
                self.accumulator = Duration::new(0, 0);
                self.update_counter = 0;
                false
            } else {
                self.accumulator -= self.update_interval;
                true
            }
        } else {
            // Not ready for another update yet
            false
        }
    }

    /// Return the amount of time (relative to the update period) since the last update tick.
    ///
    /// Also refreshes the update counter (see the `limit` method)
    pub fn render_delta(&mut self) -> f64 {
        self.update_counter = 0;
        duration_to_float(self.accumulator) / duration_to_float(self.update_interval)
    }
}

fn duration_to_float(dur: Duration) -> f64 {
    (dur.as_secs() as f64 + dur.subsec_nanos() as f64 / NANOS_PER_SECOND)
}

// Legacy macro
#[deprecated]
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
