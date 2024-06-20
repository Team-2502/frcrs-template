use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut last_loop = Instant::now();
    let mut dt = Duration::from_millis(0);
    loop {
        {
        dt = last_loop.elapsed();
        let elapsed = dt.as_secs_f64();
        let left = (1. / 500. - elapsed).max(0.);
        sleep(Duration::from_secs_f64(left)).await;
        println!("{}", 1. / last_loop.elapsed().as_secs_f64());
        last_loop = Instant::now();
        }
    }
}
