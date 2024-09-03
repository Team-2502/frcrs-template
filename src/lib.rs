pub mod subsystems;
pub mod constants;

use std::cell::RefCell;
use std::rc::Rc;
use frcrs::container;
use frcrs::input::Joystick;
use tokio::task::LocalSet;
use crate::subsystems::{Drivetrain, Shooter};
use frcrs::sleep_hz;

pub struct Ferris {
    drivetrain: Rc<RefCell<Drivetrain>>,
    shooter: Rc<RefCell<Shooter>>
}

pub struct Controllers {
    left_drive: Joystick,
    right_drive: Joystick,
    operator: Joystick
}

pub async fn configure(executor: &LocalSet) {
    let ferris = Ferris {
        drivetrain: Rc::new(RefCell::new(Drivetrain::new())),
        shooter: Rc::new(RefCell::new(Shooter::new())),
    };

    let mut controllers = Controllers {
        left_drive: Joystick::new(constants::input::LEFT),
        right_drive: Joystick::new(constants::input::RIGHT),
        operator: Joystick::new(constants::input::OPERATOR),
    };

    container!(container, executor, &ferris, &mut controllers);
}

pub async fn container<'a>(executor: &'a LocalSet, ferris: &Ferris, controllers: &mut Controllers) {
    if let Ok(mut drivetrain) = ferris.drivetrain.try_borrow_mut() {
        drivetrain.drive(controllers.left_drive.get_y(), controllers.right_drive.get_y());
    }

    if let Ok(shooter) = ferris.shooter.clone().try_borrow_mut() {
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
    }

    if controllers.operator.get(1) {
        let shooter = ferris.shooter.clone();

        executor.spawn_local(async move {
            if let Ok(shooter) = shooter.try_borrow() {
                shooter.shoot().await;
            }
        });
    }
}