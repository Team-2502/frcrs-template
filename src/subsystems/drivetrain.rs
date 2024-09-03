use frcrs::ctre::SRX;
use crate::constants;

pub struct Drivetrain {
    front_left: SRX,
    back_left: SRX,
    front_right: SRX,
    back_right: SRX
}

impl Drivetrain {
    pub fn new() -> Self {
        Self {
            front_left: SRX::new(constants::drivetrain::FRONT_LEFT),
            back_left: SRX::new(constants::drivetrain::BACK_LEFT),
            front_right: SRX::new(constants::drivetrain::FRONT_RIGHT),
            back_right: SRX::new(constants::drivetrain::BACK_RIGHT),
        }
    }

    pub fn drive(&self, left: f64, right: f64) {
        self.front_left.set(-left);
        self.back_left.set(-left);

        self.front_right.set(-right);
        self.back_right.set(-right);
    }

    pub fn stop(&self) {
        self.front_left.set(0.);
        self.back_left.set(0.);
        self.front_right.set(0.);
        self.back_right.set(0.);
    }
}