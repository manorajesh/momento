use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// A struct that represents a watch which keeps track of the start time, offset, and meridiem.
#[derive(Debug, Clone, Copy, Default)]
pub struct Watch {
    /// the starting time of the watch. This won't change over the course of the program
    pub start: i32,

    /// the offset of the watch or the time span that will be added to the start time
    pub offset: i32,

    /// whether the watch is in 12h or 24h format (true for 12h)
    pub meridiem: bool,
}

impl Watch {
    /// create a new watch with the given time and meridiem option
    pub fn new(time: &str, meridiem: bool) -> Self {
        let secs = Watch::str_to_secs(time, true);
        Watch {
            start: secs,
            meridiem,
            ..Default::default()
        }
    }

    /// take a time string (e.g. "01:23:45 AM") and return the number of seconds
    pub fn str_to_secs(time: &str, is_time_span: bool) -> i32 {
        let pm = {
            let time = time.replace('.', "").to_uppercase();
            time.contains("PM") && is_time_span
        };
        let mut time = time.split(' ').next().unwrap_or("").split(':');
        let mut hours = time.next().unwrap_or("").parse::<i32>().unwrap_or(0);
        let minutes = time.next().unwrap_or("").parse::<i32>().unwrap_or(0);
        let seconds = time.next().unwrap_or("").parse::<i32>().unwrap_or(0);

        if pm {
            hours += 12;
        }
        hours * 3600 + minutes * 60 + seconds
    }

    /// convert secs to string (HH:MM:SS format)
    pub fn secs_to_mil(secs: i32) -> String {
        let hours = secs / 3600 % 24;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    /// convert secs to string (12h format)
    pub fn secs_to_mer(secs: i32) -> String {
        let mut hours = secs / 3600 % 24;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        let meridiem = if hours >= 12 {
            hours %= 12;
            "PM"
        } else {
            "AM"
        };

        if hours == 0 {
            hours = 12;
        }

        format!("{:02}:{:02}:{:02} {}", hours, minutes, seconds, meridiem)
    }

    /// convert diff seconds to num of days later or before
    pub fn diff_to_days(diff: i32) -> String {
        let days = diff / 84600;
        match days.cmp(&0) {
            std::cmp::Ordering::Greater => format!(" +{} days", days),
            std::cmp::Ordering::Less => format!(" -{} days", days.abs()),
            std::cmp::Ordering::Equal => "".to_string(),
        }
    }

    /// return the end time of the watch
    pub fn add_offset(&self) -> i32 {
        self.start + self.offset
    }
}

// Operations
impl Add<i32> for Watch {
    type Output = Watch;

    fn add(self, rhs: i32) -> Self::Output {
        Watch {
            offset: self.offset + rhs,
            ..self
        }
    }
}

impl Sub<i32> for Watch {
    type Output = Watch;

    fn sub(self, rhs: i32) -> Self::Output {
        Watch {
            offset: self.offset - rhs,
            ..self
        }
    }
}

impl Add<&str> for Watch {
    type Output = Watch;

    fn add(self, rhs: &str) -> Self::Output {
        let secs = Watch::str_to_secs(rhs, false);
        self + secs
    }
}

impl Sub<&str> for Watch {
    type Output = Watch;

    fn sub(self, rhs: &str) -> Self::Output {
        let secs = Watch::str_to_secs(rhs, false);
        self - secs
    }
}

// Custom trait that will be implemented by i32 and &str
trait AddableToWatch {}

// Implementing the trait for i32 and &str
impl AddableToWatch for i32 {}
impl AddableToWatch for &str {}

impl<T: AddableToWatch + Copy> AddAssign<T> for Watch
where
    Watch: Add<T, Output = Watch>,
{
    fn add_assign(&mut self, other: T) {
        *self = *self + other;
    }
}

impl<T: AddableToWatch + Copy> SubAssign<T> for Watch
where
    Watch: Sub<T, Output = Watch>,
{
    fn sub_assign(&mut self, other: T) {
        *self = *self - other;
    }
}

// Display and Formatting
impl fmt::Display for Watch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let end = if self.add_offset() < 0 {
            86400 + self.add_offset()
        } else {
            self.add_offset()
        };

        let diff = self.add_offset() - self.start;

        let end_str = if self.meridiem {
            Watch::secs_to_mer(end)
        } else {
            Watch::secs_to_mil(end)
        };

        let diff_str = Watch::diff_to_days(diff);

        write!(f, "{}{}", end_str, diff_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::Watch;

    #[test]
    fn basic_adding() {
        let mut watch = Watch::new("13:33:23", false);
        watch += "0:23:03";
        assert_eq!(format!("{}", watch), "13:56:26");

        watch += 1000;
        assert_eq!(format!("{}", watch), "14:13:06");
    }

    #[test]
    fn basic_subtracting() {
        let mut watch = Watch::new("13:33:23", false);
        watch -= "0:23:03";
        assert_eq!(format!("{}", watch), "13:10:20");

        watch -= 1000;
        assert_eq!(format!("{}", watch), "12:53:40");
    }

    #[test]
    fn basic_meridiem_adding() {
        let mut watch = Watch::new("01:33:23 PM", true);
        watch += "0:23:03";
        assert_eq!(format!("{}", watch), "01:56:26 PM");

        watch += 1000;
        assert_eq!(format!("{}", watch), "02:13:06 PM");
    }

    #[test]
    fn basic_meridiem_subtracting() {
        let mut watch = Watch::new("13:33:23", true);
        watch -= "0:23:03";
        assert_eq!(format!("{}", watch), "01:10:20 PM");

        watch -= 1000;
        assert_eq!(format!("{}", watch), "12:53:40 PM");
    }

    #[test]
    fn day_overflow_adding() {
        let mut watch = Watch::new("13:33:23", false);
        watch += "23:44:03";
        assert_eq!(format!("{}", watch), "13:17:26 +1 days");

        watch += 7989;
        assert_eq!(format!("{}", watch), "15:30:35 +1 days");
    }

    #[test]
    fn day_overflow_subtracting() {
        let mut watch = Watch::new("13:33:23", false);
        watch -= "23:44:03";
        assert_eq!(format!("{}", watch), "13:49:20 -1 days");

        watch -= 7989;
        assert_eq!(format!("{}", watch), "11:36:11 -1 days");
    }

    #[test]
    fn day_overflow_meridiem_adding() {
        let mut watch = Watch::new("01:33:23 PM", true);
        watch += "23:44:03";
        assert_eq!(format!("{}", watch), "01:17:26 PM +1 days");

        watch += 7989;
        assert_eq!(format!("{}", watch), "03:30:35 PM +1 days");
    }

    #[test]
    fn day_overflow_meridiem_subtracting() {
        let mut watch = Watch::new("13:33:23", true);
        watch -= "23:44:03";
        assert_eq!(format!("{}", watch), "01:49:20 PM -1 days");

        watch -= 7989;
        assert_eq!(format!("{}", watch), "11:36:11 AM -1 days");
    }
}
