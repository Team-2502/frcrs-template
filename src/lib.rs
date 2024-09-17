pub mod subsystems;
pub mod constants;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;
use frcrs::container;
use frcrs::input::Joystick;
use tokio::task::LocalSet;
use crate::subsystems::{Drivetrain, Shooter};
use frcrs::sleep_hz;
use frcrs::Subsystem;

pub struct Ferris {
    drivetrain: Subsystem<Drivetrain>,
    shooter: Subsystem<Shooter>
}

pub struct Controllers {
    left_drive: Joystick,
    right_drive: Joystick,
    operator: Joystick
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

    container!(container, executor, &ferris, &mut controllers);
}

pub async fn container<'a>(executor: &'a LocalSet, ferris: &Ferris, controllers: &mut Controllers) {
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
        executor.spawn_local(async move {
            ferris.shooter.clone().with_borrow_mut_async(|shooter| async {
                shooter.shoot().await;
            }).await; // Might break?
        });
    }
}