#![allow(dead_code)]

use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use std::{collections::HashMap, vec};
use tailcall::tailcall;
use thiserror::Error;

/// ## Conventions:
/// - D11 means *desirability*. These are indexable ranges of time of how much the user is willing to use that time, in inverse order – the lowest the index,
/// the better for the user to use that time, starting at 0.

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DateTimeRange {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

#[derive(Clone, Copy)]
struct TimeRange {
    start: NaiveTime,
    end: NaiveTime,
}

type TimeD11Index = Vec<Vec<TimeRange>>;
type WeeklyD11Map = HashMap<Weekday, TimeD11Index>;

type DateTimeD11Index = Vec<Vec<DateTimeRange>>;

struct UserCfg {
    weekly_d11: WeeklyD11Map,
}

impl Default for UserCfg {
    /// all days from 7 to 22.
    fn default() -> Self {
        let all_weekdays = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        let time_range = TimeRange {
            start: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
        };
        let pairs = all_weekdays
            .into_iter()
            .map(|weekday| (weekday, vec![vec![time_range]]));
        Self {
            weekly_d11: HashMap::from_iter(pairs),
        }
    }
}

struct Cycle {
    start_date: NaiveDate,
    duration: Duration,
}

struct Cycles {
    cycles: Vec<Cycle>,
    breaks: Vec<Cycle>,
}

/// The key represents an increasing-number from zero, symbolising the "desirabilty" of a time range.
struct ScheduleOpts {
    d11: DateTimeD11Index,
}

#[derive(Debug, Error)]
enum CycleCfgErr {
    #[error("No cycles are configured")]
    NotCycleConfigured,
}

impl ScheduleOpts {
    /// Uses user's default desirability.
    pub fn weekends_in_clycle(cycle: &Cycle, usr_d11: WeeklyD11Map) -> Self {
        Self::build_from_usr_cfg(cycle, usr_d11, |weekday| {
            vec![Weekday::Sat, Weekday::Sun].contains(&weekday)
        })
    }

    pub fn weekdays_in_cycle(cycle: &Cycle, usr_d11: WeeklyD11Map) -> Self {
        Self::build_from_usr_cfg(cycle, usr_d11, |weekday| {
            vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
            ]
            .contains(&weekday)
        })
    }

    pub fn all_in_cycle(cycle: &Cycle, usr_d11: WeeklyD11Map) -> Self {
        Self::build_from_usr_cfg(cycle, usr_d11, |_| true)
    }

    fn build_from_usr_cfg(
        cycle: &Cycle,
        usr_d11: WeeklyD11Map,
        is_weekday: impl (Fn(Weekday) -> bool),
    ) -> Self {
        let mut d11: DateTimeD11Index = Vec::with_capacity(20);
        for day in cycle
            .start_date
            .iter_days()
            .take(cycle.duration.num_days().try_into().unwrap())
        {
            if is_weekday(day.weekday()) {
                let weekday_d11 = usr_d11.get(&day.weekday()).unwrap().to_owned();
                for (d11_index, time_ranges) in weekday_d11.into_iter().enumerate() {
                    let mut date_time_ranges: Vec<_> = time_ranges
                        .into_iter()
                        .map(|time_range| DateTimeRange {
                            start: NaiveDateTime::new(day, time_range.start),
                            end: NaiveDateTime::new(day, time_range.end),
                        })
                        .collect();

                    if let Some(idx) = d11.get_mut(d11_index) {
                        idx.append(&mut date_time_ranges);
                    } else {
                        d11.insert(d11_index, date_time_ranges);
                    }
                }
            }
        }

        d11.iter_mut().for_each(|idx| idx.sort());

        Self { d11 }
    }
}

enum Interval {
    Weekly,
    Dayly,
    PerCycle,
}

enum Stuff {
    Habit {
        estimated_duration: Duration,
        times: u8,
        interval: Interval,
        schedule_options: ScheduleOpts,
    },
    Repeatable {
        min: Option<Duration>,
        desirable: Duration,
        max: Option<Duration>,
        schedule_options: ScheduleOpts,
        interval: Interval,
    },
}

#[derive(Clone, Copy)]
struct Action {
    start: NaiveDateTime,
    duration: Duration,
}

/// For the time being, just per `Cycle`.
struct Schedule {
    actions: Vec<Action>,
}

#[derive(Error, Debug)]
enum ScheduleError {
    #[error("We couldn't find a valid solution for your criteria.")]
    NotValidSolutionFound,
}

impl Schedule {
    fn schedule(cycle: &Cycle, stuff: Vec<Stuff>) -> Result<Self> {
        #[tailcall]
        fn inner_schedule() {}

        // Naive solution: fixes the current item as the first to be set, and set the rest on the available spots.
        let mut outer_solutions: Vec<Vec<Action>> = Vec::new();
        for outer_stf in &stuff {
            let mut inner_solutions: Vec<Action> = Vec::new();
            for inner_stf in &stuff {}
        }

        if outer_solutions.is_empty() {
            Err(ScheduleError::NotValidSolutionFound.into())
        } else {
            Ok(Self {
                actions: outer_solutions.first().unwrap().to_owned(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code, unreachable_code, unused)]

    use chrono::DateTime;

    use super::*;

    #[test]
    fn simple_valid_schedule() -> Result<()> {
        let cycle = Cycle {
            start_date: NaiveDate::from_ymd_opt(2023, 1, 16).unwrap(),
            duration: Duration::weeks(2),
        };
        let cycles = Cycles {
            cycles: vec![cycle],
            breaks: Vec::new(),
        };
        let usr_cfg = UserCfg::default();
        let clean_litter = Stuff::Habit {
            estimated_duration: Duration::minutes(15),
            times: 1,
            interval: Interval::Dayly,
            schedule_options: ScheduleOpts::all_in_cycle(
                cycles.cycles.get(0).unwrap(),
                usr_cfg.weekly_d11,
            ),
        };
        let stuff: Vec<Stuff> = vec![clean_litter];

        Schedule::schedule(cycles.cycles.first().unwrap(), stuff)?;

        Ok(())
    }
}
