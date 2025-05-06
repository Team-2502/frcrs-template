use std::time::{Duration, Instant};
use tokio::time::sleep;

use tokio::{runtime::Runtime, task::LocalSet};
use frcrs::{init_hal, hal_report, observe_user_program_starting, refresh_data};
use frcrs::input::RobotState;
use robotcode::{teleop, Ferris, Joysticks};

fn main() {
    println!("main");
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();

    let controller = local.run_until(async {
        if !init_hal() {
            panic!("Failed to initalize HAL");
        }
        println!("here");
        hal_report(2, 7, 0, "2024.2.1".to_string());

        observe_user_program_starting();
        let mut ferris = Ferris::new();
        let mut joysticks = Joysticks::new();

        let mut last_loop = Instant::now();

            loop {
                println!("loop");
                refresh_data();
                let elapsed = last_loop.elapsed().as_secs_f64();
                let left = (1. / 500. - elapsed).max(0.);
                sleep(Duration::from_secs_f64(left)).await;
                //println!("{}", 1. / last_loop.elapsed().as_secs_f64());
                last_loop = Instant::now();
                let state = RobotState::get();
                if state.enabled() && state.teleop() {
                    teleop(&mut ferris, &mut joysticks);
                    println!("teleop");
                }
                
                
            }
    });

    executor.block_on(controller);
}
