use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock{
    minutes: i32,
}
impl Display for Clock{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes.div_euclid(60), self.minutes.rem_euclid(60))
    }
}

impl Clock {
    fn normalise(minutes: i32) -> i32 {
        let mut minutes = minutes;
        while minutes >= 1440 {
            minutes = minutes.rem_euclid(1440);
        }
        while minutes < 0 {
            minutes += 1440;
        }
        minutes
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let minutes = Self::normalise(hours*60+minutes);
        Clock {
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // unimplemented!("Add {minutes} minutes to existing Clock time");
        let minutes = Self::normalise(self.minutes + minutes);
        Clock {
            minutes: minutes,
        }
    }
}
