use std::fmt;

const ONE_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from_minutes(hours * 60 + minutes)
    }

    fn to_minutes(&self) -> i32 {
        self.hours * 60 + self.minutes
    }

    fn from_minutes(minutes: i32) -> Self {
        let minutes = minutes.rem_euclid(ONE_DAY);
        Clock {
            hours: (minutes / 60) % 24,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::from_minutes(self.to_minutes() + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
