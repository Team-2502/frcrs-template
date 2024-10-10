use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use crate::Ferris;

#[derive(Serialize, Deserialize)]
pub enum Auto {
    Zero,
    One,
}

impl Auto {
    pub fn from_dashboard(s: &str) -> Self {
        match s {
            "0" => Auto::Zero,
            "1" => Auto::One,
            _ => Auto::Zero,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Auto::Zero => "Zero",
            Auto::One => "One",
            _ => "none",
        }
    }

    pub fn iterator() -> Vec<Self> {
        vec![
            Auto::Zero,
            Auto::One
        ]
    }

    pub fn names() -> Vec<String> {
        Self::iterator().iter().map(|a| a.name().to_owned()).collect()
    }

    pub async fn run_auto<'a>(ferris: Ferris, chosen: Auto) {
        match chosen {
            Auto::Zero => auto_zero().await,
            Auto::One => auto_one().await,
        }
    }
}

async fn auto_zero() {
    println!("Running auto 0");
    sleep(Duration::from_secs_f64(1.)).await;
    println!("Finished auto 0");
}

async fn auto_one() {
    println!("Running auto 1");
    sleep(Duration::from_secs_f64(1.)).await;
    println!("Finished auto 1");
}