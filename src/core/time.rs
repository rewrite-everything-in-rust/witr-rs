use chrono::{DateTime, TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_duration(start_time: u64) -> (String, String) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let duration_secs = now.saturating_sub(start_time);

    let relative = match duration_secs {
        0..=59 => "just now".to_string(),
        60..=119 => "1 min ago".to_string(),
        120..=3599 => format!("{} min ago", duration_secs / 60),
        3600..=7199 => "1 hour ago".to_string(),
        7200..=86399 => format!("{} hours ago", duration_secs / 3600),
        86400..=172799 => "1 day ago".to_string(),
        _ => format!("{} days ago", duration_secs / 86400),
    };

    let datetime: DateTime<Utc> = Utc
        .timestamp_opt(start_time as i64, 0)
        .single()
        .unwrap_or_else(|| Utc::now());

    let formatted = datetime.format("%a %Y-%m-%d %H:%M:%S %z").to_string();

    (relative, formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_just_now() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let (relative, _) = format_duration(now);
        assert_eq!(relative, "just now");
    }

    #[test]
    fn test_format_minutes_ago() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let five_min_ago = now - 300;
        let (relative, _) = format_duration(five_min_ago);
        assert_eq!(relative, "5 min ago");
    }

    #[test]
    fn test_format_hours_ago() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let two_hours_ago = now - 7200;
        let (relative, _) = format_duration(two_hours_ago);
        assert_eq!(relative, "2 hours ago");
    }

    #[test]
    fn test_format_days_ago() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let three_days_ago = now - (3 * 86400);
        let (relative, _) = format_duration(three_days_ago);
        assert_eq!(relative, "3 days ago");
    }
}
