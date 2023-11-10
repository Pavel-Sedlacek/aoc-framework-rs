#[macro_export]
macro_rules! aoc_benchmark {
    ($x:expr, $y: literal) => {
        use aoc_timers::Timer;

        let mut timer = Timer::new();

        for i in 0..$y {
            $x;
            timer.lap();
        }

        let laps = timer.get_laps().iter();

        println!("Average: {:0>9}ms", laps.clone().min().unwrap().as_millis());
        println!("Worst  : {:0>9}ms", laps.clone().max().unwrap().as_millis());
        println!("Best   : {:0>9}ms", laps.clone().min().unwrap().as_millis());

    };
}