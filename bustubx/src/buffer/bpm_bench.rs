/*
1. 先解析命令行参数
*/
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

struct BpmMetrics {
    start_time: Instant,
    last_report_at: Duration,
    last_cnt: u64,
    cnt: u64,
    reporter: String,
    duration_ms: u64,
}

impl BpmMetrics {
    fn new(reporter: String, duration_ms: u64) -> Self {
        Self {
            start_time: Instant::now(),
            last_report_at: Duration::from_millis(0),
            last_cnt: 0,
            cnt: 0,
            reporter,
            duration_ms,
        }
    }

    fn tick(&mut self) {
        self.cnt += 1;
    }

    fn begin(&mut self) {
        self.start_time = Instant::now();
    }

    fn report(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.start_time);

        if elapsed.as_millis() as u64 - self.last_report_at.as_millis() as u64 > 1000 {
            println!(
                "[{:5.2}] {}: total_cnt={:<10} throughput={:<10.3} avg_throughput={:<10.3}",
                elapsed.as_secs_f64(),
                self.reporter,
                self.cnt,
                (self.cnt - self.last_cnt) as f64 / (elapsed - self.last_report_at).as_secs_f64() * 1000.0,
                self.cnt as f64 / elapsed.as_secs_f64() * 1000.0,
            );
            self.last_report_at = elapsed;
            self.last_cnt = self.cnt;
        }
    }

    fn should_finish(&self) -> bool {
        let now = Instant::now();
        now.duration_since(self.start_time).as_millis() as u64 > self.duration_ms
    }
}

fn main() {
    let metrics = Arc::new(Mutex::new(BpmMetrics::new("Reporter".to_string(), 5000)));

    let metrics_clone = Arc::clone(&metrics);
    let handle = thread::spawn(move || {
        let mut metrics = metrics_clone.lock().unwrap();
        metrics.begin();
        while !metrics.should_finish() {
            metrics.tick();
            metrics.report();
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();
}
