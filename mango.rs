use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

// Global timer counter
static TIMER: AtomicU64 = AtomicU64::new(0);

fn main() {
    // Spawn a background thread that increments the timer every second
    thread::spawn(|| {
        loop {
            TIMER.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Main loop prints the timer value
    loop {
        let t = TIMER.load(Ordering::SeqCst);
        println!("Timer = {}", t);
        thread::sleep(Duration::from_millis(500));
    }
}
