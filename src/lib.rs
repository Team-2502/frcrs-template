pub mod subsystems;
pub mod constants;
mod auto;

use std::time::Duration;
use frcrs::input::Joystick;
use tokio::task::LocalSet;
use crate::subsystems::{Drivetrain, Shooter};
use frcrs::sleep_hz;
use frcrs::Subsystem;
use frcrs::telemetry::Telemetry;
use tokio::time::sleep;
use frcrs::refresh_data;
use frcrs::input::RobotState;
use crate::auto::Auto;

#[derive(Clone)]
pub struct Ferris {
    drivetrain: Subsystem<Drivetrain>,
    shooter: Subsystem<Shooter>,
}

pub struct Controllers {
    left_drive: Joystick,
    right_drive: Joystick,
    operator: Joystick,
}

pub async fn configure(executor: &LocalSet) {
    let ferris = Ferris {
        drivetrain: Subsystem::new(Drivetrain::new()),
        shooter: Subsystem::new(Shooter::new()),
    };

    let mut controllers = Controllers {
        left_drive: Joystick::new(constants::input::LEFT),
        right_drive: Joystick::new(constants::input::RIGHT),
        operator: Joystick::new(constants::input::OPERATOR),
    };

    let telemetry = Telemetry::new(5807);

    telemetry.add_string("auto chooser", serde_json::to_string(&Auto::names()).unwrap()).await;
    telemetry.add_number("selected auto", 0).await;

    let mut auto_handle = None;

    let last_loop = std::time::Instant::now();
    loop {
        refresh_data();

        let state = RobotState::get();

        if state.enabled() && state.teleop() {
            container(executor, &ferris, &mut controllers, &telemetry).await;
        } else if state.enabled() && state.auto() {
            if auto_handle.is_none() {
                let f = ferris.clone();

                if let Some(selected_auto) = telemetry.get("selected auto").await {
                    let chosen = Auto::from_dashboard(selected_auto.as_str());

                    let auto_task = Auto::run_auto(f, chosen);
                    let handle = executor.spawn_local(auto_task).abort_handle();
                    auto_handle = Some(handle);
                } else {
                    eprintln!("Failed to get selected auto from telemetry.");
                }
            }
        } else {
            if let Some(handle) = auto_handle.take() {
                handle.abort();
            }
        }

        sleep_hz(last_loop, 500).await;
        telemetry.add_number("loop rate (hz)", (1. / last_loop.elapsed().as_secs_f64()) as i32).await;
    }
}

pub async fn container<'a>(
    executor: &'a LocalSet,
    ferris: &Ferris,
    controllers: &mut Controllers,
    telemetry: &Telemetry,
) {
    update_telemetry(telemetry).await;

    ferris.drivetrain.with_borrow_mut(|drivetrain| {
        drivetrain.drive(controllers.left_drive.get_y(), controllers.right_drive.get_y());
    });

    ferris.shooter.with_borrow_mut(|shooter| {
        if controllers.operator.get(2) {
            shooter.set_shooter(1. - (controllers.operator.get_throttle() + 1.) / 2.);
        } else {
            shooter.set_shooter(0.);
        }

        if controllers.operator.get(3) {
            shooter.set_angle(0.75);
        } else if controllers.operator.get(4) {
            shooter.set_angle(-0.75);
        } else {
            shooter.set_angle(0.);
        }
    });

    if controllers.operator.get(1) {
        let shooter = ferris.shooter.clone();

        executor.spawn_local(async move {
            if let Ok(shooter) = shooter.try_borrow_mut() {
                shooter.shoot().await;
            }
        });
    }
}

async fn update_telemetry(telemetry: &Telemetry) {
    telemetry.add_number("test", 4).await;
    sleep(Duration::from_secs_f64(1.)).await;
    if let Some(value) = telemetry.get("test").await {
        println!("Telemetry test value: {:?}", value);
    }
}
