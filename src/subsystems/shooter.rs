use std::time::Duration;
use frcrs::ctre::SRX;
use frcrs::solenoid::{ModuleType, Solenoid};
use tokio::time::sleep;
use crate::constants;

pub struct Shooter {
    front: SRX,
    back: SRX,

    angle: SRX,

    pusher: Solenoid
}

impl Shooter {
    pub fn new() -> Self {
        Self {
            front: SRX::new(constants::shooter::FRONT),
            back: SRX::new(constants::shooter::BACK),
            angle: SRX::new(constants::shooter::ANGLE),
            pusher: Solenoid::new(ModuleType::CTRE, constants::shooter::PUSHER),
        }
    }

    pub fn set_shooter(&self, speed: f64) {
        self.front.set(speed);
        self.back.set(speed);
    }

    pub fn set_angle(&self, speed: f64) {
        self.angle.set(speed);
    }

    pub async fn shoot(&self) {
        self.pusher.set(true);
        sleep(Duration::from_millis(250)).await;
        self.pusher.set(false);
    }
}