use log::error;
use serde::Serialize;

// One tick is 5 seconds of real time, and a minute of game time.
pub const TICK_INTERVAL: u64 = 5;
const HOUR: u64 = 30; // Thirty ticks represents one game hour ...
const DAY: u64 = HOUR * 24; // Twenty four hours in a day ...
const DAYS_IN_MONTH: u64 = 28;
const MONTH: u64 = DAY * DAYS_IN_MONTH; // Four weeks in a month; makes things simple ...
const MONTHS_IN_YEAR: u64 = 12;
const YEAR: u64 = MONTH * MONTHS_IN_YEAR;

#[derive(Debug, Clone, Serialize)]
pub struct DateTime {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
    day_phase: (DayPhase, Period),
    season: (Season, Period),
}

impl From<Clock> for DateTime {
    fn from(clock: Clock) -> DateTime {
        DateTime {
            year: clock.year(),
            month: clock.month(),
            day: clock.day_of_month(),
            hour: clock.hour(),
            minute: clock.minute(),
            day_phase: clock.phase_of_day(),
            season: clock.season(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct Clock {
    pub tick: u64,
}

impl Clock {
    pub fn new(tick: u64) -> Self {
        Clock { tick }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn dump(&self) -> String {
        format!(
            "{:02}:{:02} {:?} (month: {}, year: {}, season: {:?})",
            self.hour(),
            self.minute(),
            self.phase_of_day(),
            self.month(),
            self.year(),
            self.season()
        )
    }

    pub fn year(&self) -> u64 {
        self.tick / YEAR
    }

    pub fn tick_of_year(&self) -> u64 {
        self.tick % YEAR
    }

    pub fn tick_of_day(&self) -> u64 {
        self.tick % DAY
    }

    pub fn minute_of_day(&self) -> u64 {
        self.tick_of_day() * (60 / HOUR)
    }

    pub fn minute(&self) -> u64 {
        self.minute_of_day() % 60
    }

    pub fn hour(&self) -> u64 {
        self.minute_of_day() / 60
    }

    pub fn phase_of_day(&self) -> (DayPhase, Period) {
        let basis = DAY / 12;
        match self.tick_of_day() / basis {
            0 => (DayPhase::Night, Period::Mid),
            1 => (DayPhase::Night, Period::Late),
            2 => (DayPhase::Morning, Period::Early),
            3 => (DayPhase::Morning, Period::Mid),
            4 => (DayPhase::Morning, Period::Late),
            5 => (DayPhase::Day, Period::Early),
            6 => (DayPhase::Day, Period::Mid),
            7 => (DayPhase::Day, Period::Late),
            8 => (DayPhase::Evening, Period::Early),
            9 => (DayPhase::Evening, Period::Mid),
            10 => (DayPhase::Evening, Period::Late),
            11 => (DayPhase::Night, Period::Early),
            d => {
                error!(
                    "Clock.phase_of_day() -- Tick {} -- tick_of_day {} with basis {}",
                    self.tick, d, basis
                );
                (DayPhase::Night, Period::Mid)
            } // WHAT?? NEVER!!
        }
    }

    pub fn day_of_year(&self) -> u64 {
        self.tick_of_year() / DAY
    }

    pub fn month(&self) -> u64 {
        self.day_of_year() / DAYS_IN_MONTH
    }

    pub fn day_of_month(&self) -> u64 {
        self.day_of_year() % DAYS_IN_MONTH
    }

    pub fn season(&self) -> (Season, Period) {
        match self.month() {
            0 => (Season::Winter, Period::Mid),
            1 => (Season::Winter, Period::Late),
            2 => (Season::Spring, Period::Early),
            3 => (Season::Spring, Period::Mid),
            4 => (Season::Spring, Period::Late),
            5 => (Season::Summer, Period::Early),
            6 => (Season::Summer, Period::Mid),
            7 => (Season::Summer, Period::Late),
            8 => (Season::Autumn, Period::Early),
            9 => (Season::Autumn, Period::Mid),
            10 => (Season::Autumn, Period::Late),
            11 => (Season::Winter, Period::Early),
            m => {
                error!("Clock.season() -- Tick {} -- month {}", self.tick, m);
                (Season::Winter, Period::Mid)
            } // WHAT?? NEVER!!
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Period {
    Early,
    Mid,
    Late,
}

#[derive(Debug, Clone, Serialize)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

#[derive(Debug, Clone, Serialize)]
pub enum DayPhase {
    Morning,
    Day,
    Evening,
    Night,
}
