use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours as f32 + minutes as f32 / 60.0).rem_euclid(24.0) as i32,
            minutes: (minutes as f32).rem_euclid(60.0) as i32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(
            (self.hours as f32 + ((self.minutes as f32 + minutes as f32) / 60.0)).rem_euclid(24.0)
                as i32,
            (self.minutes as f32 + minutes as f32).rem_euclid(60.0) as i32,
        )
    }
}
