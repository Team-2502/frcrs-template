use frcrs::sleep_hz;
use frcrs::telemetry::Telemetry;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let telemetry = Telemetry::new(5807);

    telemetry.add_number("test", 4).await;
    println!("test: {:?}", telemetry.get("test").await);

    telemetry.add_string("auto chooser", serde_json::to_string(&Auto::names()).unwrap()).await;
    telemetry.add_number("selected auto", 0).await;
    telemetry.add_number("loop rate (hz)", 1295).await;

    let mut prev = telemetry.get("selected auto").await.unwrap();

    let mut instant = Instant::now();

    loop {
        let new = telemetry.get("selected auto").await.unwrap();
        if prev != new {
            println!("Auto updated from {} to {}", prev, new);
            prev = new;
        }

        sleep_hz(instant, 50).await;
    }
}

#[derive(Serialize, Deserialize)]
pub enum Auto {
    Zero,
    One,
}

impl Auto {
    pub fn from_dashboard(s: &str) -> Self {
        match s {
            "Zero" => Auto::Zero,
            "One" => Auto::One,
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
}
