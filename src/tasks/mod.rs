//! Daemon DOES NOT allow "except" statements which are days, only hours. Days 
//! define the schedule, and "except" just says "not between these hours"...
//! 
//! Input types:
//!  - Mon - Always just one day,
//!  - Mon/Tue - Just one day, either one or the other, depending on when the 
//!  - lowest carbon emissions are (if this were Mon/Tue,Wed/Thu it could be run 
//!  two days in a row),
//!  - Mon..Thu - Several days in succession,
//!  - Mon,Tue,Wed - Several days, not in succession (could be Mon/Tue,Wed/Thu 
//!  but not Mon/Tue,Tue/Wed, as that could cause it to run twice in one day)
//! 
//! At midnight of each day (or as soon as it is launched, if it hasn't been 
//! launched today), it runs a query for the 48hr forecast, and schedules the 
//! daily tasks (based on the start day if it is an optional/one-of-two days).
//! 
//! Matching to the forecast:
//!  - The forecast is called once per day and returns a minimal list of start 
//! times and end times for hours over the next 48 hour period and their 
//! corresponding carbon intensity.
//!  - Today == Today, tomorrow is only relevant if the job at today is an 
//! extended day,
//! - Hour = "Except Hour", where if the "Except Hour" bit is set for a schedule, 
//! the next best carbon intensity hour is found , the application must loop 
//! through the schedule if `"Except Hour" & best hour` (bitwise &), if the 
//! "Except Hour" bitmask == 0, then we know we can just go with the lowest in 
//! the given period (this means that Except must have exactly the same bitmask 
//! values as those use to define hours response from the carbon intensity API).
//!
//! From testing, the NESO Carbon Intensity API behaviour will always return 
//! periods in 30-minute intervals, on the 30th minute. The forecast doesn't 
//! vary based on the time requested (2026-05-04T00:00:00-00:30:00 will always 
//! be the same intensity, whether it is the first or 10th period returned). The 
//! request period is always inclusive in the first period returned (e.g. if you 
//! request, 00:00:00, the first period will be 23:30-00:00). Seconds are ignored 
//! completely, and are not actually required for the request (2026-05-04T00:00Z 
//! will work, and 2026-05-04T00:00:59Z will be treat as 2026-05-04T00:00:00Z and 
//! will return an initial period ending with 00:00).
//! 
//! 
//! Postcodes > Requests >  

use std::ffi::OsString;
use std::str::FromStr;

pub type PostcodeVec = Vec<String>;

#[derive(Debug)]
pub struct Service {
    name: String,
    // Known location for the service definition.
    known_loc: OsString
}

#[derive(Debug)]
pub struct ScheduledTask {
    // Index for the Postcode in the PostcodeVec, useful for machines based in 
    // one location, but calling remote functions on machines in another 
    // location (e.g. if the remote machine cannot send/recieve data outside the 
    // network, so must be scheduled from a machine that can access the Carbon 
    // Intensity API).
    loc: usize,
    task: Service
}

// Each period and its correspondgin "extended" should be thought of as similar 
// to an AoS data structure, if the "ext" bit is set for the corresponding 
// period bit, that means the lowest point of the 48hr schedule should be 
// used for the schedule.
#[derive(Debug)]
pub struct Schedule {
    days: StartDays,
    day_exts: OptionalDays,
    task: ScheduledTask
}

///
#[derive(Debug)]
pub struct OptionalDays(u8);


#[derive(Debug)]
pub struct StartDays(u8);



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
        todo!()
        // match s.to_lowercase()
    }
}

// #[derive(Debug)]
// pub struct Schedule {
//     weekdays: WeekdaySet,
//     days_of_month: u32, // 1u8 << 31 is the last day of the month.
//     months: u16,
//     years: Vec<i16>,
//     hours: u16,
//     minutes: u64,
//     seconds: u64
// }

// https://www.man7.org/linux/man-pages/man7/systemd.time.7.html

//                minutely → *-*-* *:*:00
//                  hourly → *-*-* *:00:00
//                   daily → *-*-* 00:00:00
//                 monthly → *-*-01 00:00:00
//                  weekly → Mon *-*-* 00:00:00
//                  yearly → *-01-01 00:00:00
//               quarterly → *-01,04,07,10-01 00:00:00
//            semiannually → *-01,07-01 00:00:00