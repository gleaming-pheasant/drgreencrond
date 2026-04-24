use std::str::FromStr;

bitflags::bitflags! {
    /// Represents a set of flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Weekday: u8 {
        const Sun = 0b00000001;
        const Mon = 0b00000010;
        const Tue = 0b00000100;
        const Wed = 0b00001000;
        const Thu = 0b00010000;
        const Fri = 0b00100000;
        const Sat = 0b01000000;
    }
}

impl FromStr for Weekday {
    type Err = anyhow::Error;
    /// This impl assumes the conversion to lowercase has already been done on 
    /// the list of days being parsed, and this is used to parse the individual 
    /// days in the Split. This is to reduce allocations for each returned day 
    /// individually.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sun" | "sunday" => Ok(Weekday::Sun),
            "mon" | "monday" => Ok(Weekday::Mon),
            "tue" | "tuesday" => Ok(Weekday::Tue),
            "wed" | "wednesday" => Ok(Weekday::Wed),
            "thu" | "thursday" => Ok(Weekday::Thu),
            "fri" | "friday" => Ok(Weekday::Fri),
            "sat" | "saturday" => Ok(Weekday::Sat),
            _ => Err(anyhow::Error::msg("Uknown weekday"))
        }
    }
}

/// Represents a single byte representing a collection of `Weekday`s, i.e. this 
/// is where a list of `Weekday`s should be saved in a schedule.
#[derive(Debug)]
#[repr(transparent)]
pub struct WeekdaySet(Weekday);

impl FromStr for WeekdaySet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase()
    }
}

#[derive(Debug)]
pub struct Schedule {
    weekdays: WeekdaySet,
    days_of_month: u32, // 1u8 << 31 is the last day of the month.
    months: u16,
    years: Vec<i16>,
    hours: u16,
    minutes: u64,
    seconds: u64
}

// https://www.man7.org/linux/man-pages/man7/systemd.time.7.html

//                minutely → *-*-* *:*:00
//                  hourly → *-*-* *:00:00
//                   daily → *-*-* 00:00:00
//                 monthly → *-*-01 00:00:00
//                  weekly → Mon *-*-* 00:00:00
//                  yearly → *-01-01 00:00:00
//               quarterly → *-01,04,07,10-01 00:00:00
//            semiannually → *-01,07-01 00:00:00