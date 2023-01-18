#![allow(dead_code)]

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use delegate::delegate;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
    vec,
};
use thiserror::Error;
use uuid::Uuid;

/// ## Conventions:
/// - D11 means *desirability*. These are indexable ranges of time of how much the user is willing to use that time, in inverse order – the lowest the index,
/// the better for the user to use that time, starting at 0.

/// Something with a start and an end, with some useful implementations to handle durations and
/// other things.
#[derive(Debug, Clone, Copy)]
struct Range<T> {
    start: T,
    end: T,
}

impl<T> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl<T: Add<Duration, Output = T> + Clone> Range<T> {
    pub fn from_duration(start: T, duration: Duration) -> Self {
        Self {
            start: start.clone(),
            end: start + duration,
        }
    }
}

impl<T: Sub<T, Output = Duration> + Clone> Range<T> {
    pub fn duration(&self) -> Duration {
        self.end.clone().sub(self.start.clone())
    }
}

impl<T: PartialOrd> Range<T> {
    /// Does the `with` range *overlaps* with [`self`](Range)?.
    pub fn overlaps(&self, with: &Self) -> bool {
        if self.start >= with.end {
            // If it starts after action ends.
            true
        } else if with.end <= self.start {
            // If action ends before `self` start.
            true
        } else {
            false
        }
    }
}

struct NaiveDateIter {
    range: Range<NaiveDate>,
}

impl Iterator for NaiveDateIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let day = self.range.start + Duration::days(1);
        if day < self.range.end {
            self.range.start = day;
            Some(day)
        } else {
            None
        }
    }
}

impl Range<NaiveDate> {
    pub fn into_days_iter(&self) -> NaiveDateIter {
        NaiveDateIter {
            range: self.clone(),
        }
    }
}

type DateRange = Range<NaiveDate>;

type TimeRange = Range<NaiveTime>;

type DateTimeRange = Range<NaiveDateTime>;

/// The outer vector represents the index of desirability; the inner are the time-slots.
type TimeD11Index = Vec<Vec<TimeRange>>;

/// Same as [TimeD11Index], but dated.
type DateTimeD11Index = Vec<Vec<DateTimeRange>>;

/// Desirability indexes per weekday. Useful to represent, for example, an user choosing to not use
/// weekends.
type WeeklyD11Map = HashMap<Weekday, TimeD11Index>;

#[derive(Debug, Clone)]
struct UserCfg {
    weekly_d11: WeeklyD11Map,
}

impl Default for UserCfg {
    /// All days, from 7 to 22, are desirable for ≤doing things≥.
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
            .map(|weekday| (weekday, vec![vec![time_range]])); // The outer Vec is the index of desirability. That means the whole 7 to 22 time-slot is at top priority of desire.
        Self {
            weekly_d11: HashMap::from_iter(pairs),
        }
    }
}

/// I'm not sure we need this struct to even exists, but something says to me we would need to add
/// more attributes to [Cycle] that might differ from just a simple range of dates.
#[derive(Debug, Clone)]
struct Cycle {
    range: DateRange,
}

impl Cycle {
    delegate! {
        to self.range {
            pub fn duration(&self) -> Duration;
            pub fn overlaps(&self, with: &DateRange) -> bool;
        }
    }
}

#[derive(Debug, Clone)]
struct Cycles {
    cycles: Vec<Cycle>,
    breaks: Vec<DateRange>,
}

/// The key represents an increasing-number from zero, symbolising the "desirability" of a time range.
#[derive(Debug, Clone)]
struct ScheduleOpts {
    d11: DateTimeD11Index,
}

#[derive(Debug, Error)]
enum CycleCfgErr {
    #[error("No cycles are configured")]
    NotCycleConfigured,
}

/// When the user desires (and in which priority) to schedule their actions.
impl ScheduleOpts {
    /// Creates desirability slots only on weekends in the cycle from a default weekly
    /// desirability.
    pub fn weekends_in_clycle(cycle: &Cycle, usr_d11: WeeklyD11Map) -> Self {
        Self::build_from_usr_cfg(cycle, usr_d11, |weekday| {
            vec![Weekday::Sat, Weekday::Sun].contains(&weekday)
        })
    }

    /// Creates desirability slots  only on weekdays (working days) in the cycle from a
    /// default weekly desirability.
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

    /// Creates desirability slots for each day in the cycle from a default weekly desirability.
    pub fn all_in_cycle(cycle: &Cycle, usr_d11: WeeklyD11Map) -> Self {
        Self::build_from_usr_cfg(cycle, usr_d11, |_| true)
    }

    /// Helper function who builds/generate a ≤default≥ desirability `Index` of the cycle
    /// for the weekdays `is_weekday` returns true, using time-slots per weekday.
    ///
    /// This is useful because the user might already set preferences on their time-slots, and
    /// they want to just use them to match an activity with those slots.
    fn build_from_usr_cfg(
        cycle: &Cycle,
        usr_d11: WeeklyD11Map,
        is_weekday: impl (Fn(Weekday) -> bool),
    ) -> Self {
        let mut d11: DateTimeD11Index = Vec::with_capacity(20);

        for day in cycle.range.into_days_iter() {
            // Skips if we're not talking about the same weekday on the default slots.
            if is_weekday(day.weekday()) {
                continue;
            }

            // What are the time-slots (by indexable priority) the user has specified for the current weekday?.
            let weekday_d11 = usr_d11.get(&day.weekday()).unwrap().to_owned();
            for (d11_index, time_ranges) in weekday_d11.into_iter().enumerate() {
                // Time ranges provided by the user are related only to a weekday; they're just that, times.
                // So we add the current day in the cycle as the date the time-slot will apply.
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

        Self { d11 }
    }
}

#[derive(Debug, Clone)]
enum Interval {
    Weekly,
    Dayly,
    PerCycle,
}

#[derive(Debug, Clone)]
struct Stuff {
    id: Uuid,
    kind: StuffKinds,
}

impl PartialEq for Stuff {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
enum StuffKinds {
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
        interval: Interval,
        schedule_options: ScheduleOpts,
    },
}

#[derive(Debug, Clone, Copy)]
struct Action {
    range: DateTimeRange,
}

impl Action {
    delegate! {
        to self.range {
            pub fn duration(&self) -> Duration;
            pub fn overlaps(&self, with: &DateTimeRange) -> bool;
        }
    }
}

/// For the time being, an schedule is tight to a [`cycle`](Cycle).
#[derive(Debug, Clone)]
struct Schedule {
    cycle: Cycle,
    actions: HashMap<Stuff, Action>,
}

impl Schedule {
    fn new(cycle: Cycle) -> Self {
        Schedule {
            cycle,
            actions: HashMap::new(),
        }
    }
}

#[derive(Error, Debug)]
enum ScheduleError {
    #[error("We couldn't find a valid solution for your criteria.")]
    NoValidSolutionFound,
}

impl Schedule {
    pub fn schedule(self, stuff: Vec<Stuff>) -> Vec<Self> {
        fn inner_schedule(left: Vec<Stuff>, candidate: Schedule, solutions: &mut Vec<Schedule>) {
            if left.is_empty() {
                if candidate.is_valid() {
                    solutions.push(candidate);
                }
            }

            for s in left {}

            todo!()
        }

        let mut solutions = Vec::new();
        inner_schedule(stuff, self, &mut solutions);
        solutions
    }

    pub fn is_valid(&self) -> bool {
        todo!("I need to know if the schedule allocated satisfies all the constraints given set into Stuff")
    }

    pub fn schedule_single(self, stuff: Stuff) -> Option<Self> {
        match stuff.kind {
            StuffKinds::Habit {
                estimated_duration,
                times,
                interval,
                schedule_options,
            } => match interval {
                Interval::Weekly => {
                    for dtr in schedule_options.d11.iter().flatten() {
                        let candidate = DateTimeRange {
                            start: dtr.start.clone(),
                            end: dtr.start.add(estimated_duration).clone(),
                        };
                        // self.actions
                        //     .values()
                        //     .filter(|a| candidate.action_collides(a));
                    }
                }
                Interval::Dayly => todo!(),
                Interval::PerCycle => todo!(),
            },
            StuffKinds::Repeatable {
                min,
                desirable,
                max,
                interval,
                schedule_options,
            } => todo!(),
        };

        todo!()
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code, unreachable_code, unused)]

    use chrono::DateTime;

    use super::*;

    #[test]
    fn simple_valid_schedule() {
        let usr_cfg = UserCfg::default();
        let cycle = Cycle {
            range: Range::from_duration(
                NaiveDate::from_ymd_opt(2023, 1, 16).unwrap(),
                Duration::weeks(2),
            ),
        };
        let cycles = Cycles {
            cycles: vec![cycle.clone()],
            breaks: Vec::new(),
        };

        let work = Stuff {
            id: Uuid::new_v4(),
            kind: StuffKinds::Repeatable {
                min: Some(Duration::hours(5)),
                desirable: Duration::hours(7),
                max: Some(Duration::hours(10)),
                interval: Interval::Dayly,
                schedule_options: ScheduleOpts::weekdays_in_cycle(
                    &cycle,
                    usr_cfg.clone().weekly_d11,
                ),
            },
        };
        let clean_litter = Stuff {
            id: Uuid::new_v4(),
            kind: StuffKinds::Habit {
                estimated_duration: Duration::minutes(15),
                times: 1,
                interval: Interval::Dayly,
                schedule_options: ScheduleOpts::all_in_cycle(
                    cycles.cycles.get(0).unwrap(),
                    usr_cfg.clone().weekly_d11,
                ),
            },
        };

        let stuff: Vec<Stuff> = vec![clean_litter, work];
        // assert!(!Schedule::schedule(stuff).is_empty(), "no solution found");
    }
}
