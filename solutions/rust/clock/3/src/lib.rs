use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock{
    // hours: i32,
    minutes: i32,
}
impl Display for Clock{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes/60, self.minutes%60)
    }
}

impl Clock {
    // fn normalise(hours: i32, minutes: i32) -> (i32, i32) {
    //     let mut hours = hours;
    //     let mut minutes = minutes;
    //     if minutes > 0 {
    //         hours += minutes / 60;
    //         minutes = minutes % 60;
    //     }
    //     while minutes < 0 {
    //         hours -= 1;
    //         minutes += 60;
    //     }
    //     while hours < 0 {
    //         hours += 24;
    //     }
    //     if hours >= 24 {
    //         hours = hours % 24;
    //     }
    //     (hours, minutes)
    // }
    fn normalise(minutes: i32) -> i32 {
        let mut minutes = minutes;
        while minutes >= 1440 {
            minutes = minutes % 1440;
        }
        while minutes < 0 {
            minutes += 1440;
        }
        minutes
    }
    // pub fn new(hours: i32, minutes: i32) -> Self {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        // let (hours, minutes) = Self::normalise(hours, minutes);
        let minutes = Self::normalise(hours*60+minutes);
        Clock {
            // hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // unimplemented!("Add {minutes} minutes to existing Clock time");
        // handle negative minutes and overflows
        // let (hours, minutes) = Self::normalise(self.hours, self.minutes + minutes);
        let minutes = Self::normalise(self.minutes + minutes);
        Clock {
            // hours: hours,
            minutes: minutes,
        }
    }
}
