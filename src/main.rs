use deadlock::deadlock;
use livelock::livelock;

fn main() {
    // deadlock();
    livelock();
}
mod livelock;

mod deadlock;
