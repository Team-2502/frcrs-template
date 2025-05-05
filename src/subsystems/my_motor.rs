use crate::constants::robotmap;
use frcrs::ctre::{ControlMode, Talon};

pub struct Motor {
    motor: Talon,
}

impl Default for Motor {
    fn default() -> Self {
        Self::new()
    }
}

impl Motor {
    pub fn new() -> Self {
        Self {
            motor: Talon::new(robotmap::my_motor::MOTOR_ID, None),
        }
    }

    pub fn set(&self, speed: f64) {
        self.motor.set(ControlMode::Percent, speed);
    }

    pub fn stop(&self) {
        self.motor.stop();
    }
}
