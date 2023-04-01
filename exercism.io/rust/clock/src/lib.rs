use std::fmt::Debug;
use std::fmt::Display;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Clock")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish()
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Eq for Clock {}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let neg_min = minutes < 0;

        let dif_hours = if neg_min {
            minutes / 60 + if -1 * minutes % 60 > 0 { -1 } else { 0 }
        } else {
            minutes / 60
        };

        let upd_mins = (60 * 1_1337 + minutes) % 60;

        Clock {
            hours: (24 * 1_1337 + hours + dif_hours) % 24,
            minutes: upd_mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mins = self.minutes + minutes;

        let neg_min = mins < 0;
        let dif_hours = if neg_min {
            mins / 60 + if -1 * mins % 60 > 0 { -1 } else { 0 }
        } else {
            mins / 60
        };

        let upd_mins = (60 * 1_1337 + mins) % 60;

        let upd_hours = (24 * 1_1337 + self.hours + dif_hours) % 24;
        Clock {
            hours: upd_hours,
            minutes: upd_mins,
        }
    }
}
