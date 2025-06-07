const TICKS_PER_MINUTE: u64 = 60;
const MINUTES_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const MINUTES_PER_DAY: u64 = HOURS_PER_DAY * MINUTES_PER_HOUR;
#[allow(unused)]
const TICKS_PER_DAY: u64 = TICKS_PER_MINUTE * MINUTES_PER_DAY;

/// (day, hour, minute)
pub fn game_time(timestamp: f64) -> (u64, u64, u64) {
    let total_minutes = (timestamp as u64) * 15;
    let day = total_minutes / MINUTES_PER_DAY;
    let minutes_today = total_minutes % MINUTES_PER_DAY;
    let hour = minutes_today / MINUTES_PER_HOUR;
    let minute = minutes_today % MINUTES_PER_HOUR;
    (day, hour, minute)
}

pub fn formatted_game_time(timestamp: f64) -> String {
    let (day, hour, minute) = game_time(timestamp);
    format!("Jour {}, {:02}h{:02}", day, hour, minute)
}
