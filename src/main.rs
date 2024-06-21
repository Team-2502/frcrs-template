use std::time::{Duration, Instant};
use tokio::time::sleep;

use tokio::{runtime::Runtime, task::LocalSet};
use frcrs::{init_hal, hal_report, observe_user_program_starting};

fn main() {
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();

    let controller = local.run_until(async {
        if !init_hal() {
            panic!("Failed to initalize HAL");
        }

        hal_report(2, 7, 0, "2024.2.1".to_string());

        observe_user_program_starting();

        let mut last_loop = Instant::now();

            loop {
                let elapsed = last_loop.elapsed().as_secs_f64();
                let left = (1. / 500. - elapsed).max(0.);
                sleep(Duration::from_secs_f64(left)).await;
                //println!("{}", 1. / last_loop.elapsed().as_secs_f64());
                last_loop = Instant::now();
            }
    });

    executor.block_on(controller);
}
