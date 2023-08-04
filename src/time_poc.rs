#![allow(dead_code, unused)]

use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use delegate::delegate;
use itertools::Itertools;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    hash::Hash,
    ops::{Add, RangeInclusive, Sub},
    vec,
};
use thiserror::Error;
use uuid::Uuid;

/// ## Conventions:
/// - D11 means *desirability*. These are indexable ranges of time of how much the user is willing to use that time, in inverse order – the lowest the index,
/// the better for the user to use that time, starting at 0.

/// A range with a start (**inclusive**) and an end (**exclusive**), with some useful implementations to handle durations and
/// other things.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

trait Timeable {
    fn duration(&self) -> Duration;
}

impl<T: Sub<T, Output = Duration> + Clone> Timeable for Range<T> {
    fn duration(&self) -> Duration {
        self.end.clone().sub(self.start.clone())
    }
}

impl<T: Add<Duration, Output = T> + Clone> Range<T>
where
    Self: Timeable,
{
    pub fn shift_to(&mut self, new_start: T) {
        let duration = self.duration();
        self.start = new_start.clone();
        self.end = new_start + duration;
    }
}

impl<T: PartialOrd> Range<T> {
    /// Does the `with` range *overlaps* with [`self`](Range)?. This property should hold
    /// backwards.
    ///
    /// [nonminimal_bool](clippy::nonminimal_bool) is allowed because I want to be expressive, not
    /// *"minimal"*.
    #[allow(clippy::nonminimal_bool)]
    pub fn overlaps(&self, with: &Self) -> bool {
        with.start < self.start && with.end > self.start
            || self.end > with.start && with.end > self.end
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
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
    pub fn iter_days(&self) -> NaiveDateIter {
        NaiveDateIter { range: *self }
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
    date_range: DateRange,
}

impl Cycle {
    delegate! {
        to self.date_range {
            pub fn duration(&self) -> Duration;
            pub fn overlaps(&self, with: &DateRange) -> bool;
            pub fn iter_days(&self) -> NaiveDateIter;
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

/// When the user desires (and in which priority) to schedule their actions.
impl ScheduleOpts {
    /// Creates desirability slots only on weekends in the cycle from a default weekly
    /// desirability.
    pub fn weekends_in_clycle(cycle: &Cycle, usr_d11: WeeklyD11Map) -> Self {
        Self::build_from_weekly_d11(cycle, usr_d11, |weekday| {
            vec![Weekday::Sat, Weekday::Sun].contains(&weekday)
        })
    }

    /// Creates desirability slots  only on weekdays (working days) in the cycle from a
    /// default weekly desirability.
    pub fn weekdays_in_cycle(cycle: &Cycle, usr_d11: WeeklyD11Map) -> Self {
        Self::build_from_weekly_d11(cycle, usr_d11, |weekday| {
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
        Self::build_from_weekly_d11(cycle, usr_d11, |_| true)
    }

    /// Helper function who builds/generate a ≤default≥ desirability `Index` of the cycle
    /// for the weekdays `is_weekday` returns true, using time-slots per weekday.
    ///
    /// This is useful because the user might already set preferences on their time-slots, and
    /// they want to just use them to match an activity with those slots.
    fn build_from_weekly_d11(
        cycle: &Cycle,
        usr_d11: WeeklyD11Map,
        is_weekday: impl (Fn(Weekday) -> bool),
    ) -> Self {
        let mut d11: DateTimeD11Index = Vec::with_capacity(20);

        for day in cycle.date_range.iter_days() {
            // Skips if we're not talking about the same weekday on the default slots.
            if is_weekday(day.weekday()) {
                continue;
            }

            // What are the time-slots (by indexable priority) the user has specified for the current weekday?.
            let weekday_d11 = usr_d11.get(&day.weekday()).unwrap().to_owned();
            for (d11_index, time_ranges) in weekday_d11.into_iter().enumerate() {
                // Time ranges provided by the user are related only to a weekday; they're just that times.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StuffId(pub Uuid);

#[derive(Debug, Clone)]
struct Stuff {
    id: StuffId,
    kind: StuffKinds,
}

impl PartialEq for Stuff {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Stuff {}

/// Hashed only by [id](StuffId).
impl Hash for Stuff {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Debug, Clone)]
enum StuffKinds {
    /// - [`times`](u8): This means times per [`interval`](Interval).
    Habit {
        estimated_duration: Duration,
        times: u8,
        interval: Interval,
        schedule_options: ScheduleOpts,
    },
    /// # Attributes
    /// - `min`, `desirable` and `max` are [durations](Duration) per [`interval`](Interval).
    Repeatable {
        min: Option<Duration>,
        desirable: Duration,
        max: Option<Duration>,
        interval: Interval,
        schedule_options: ScheduleOpts,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Action {
    dt_range: DateTimeRange,
}

impl Action {
    delegate! {
        to self.dt_range {
            pub fn duration(&self) -> Duration;
            pub fn overlaps(&self, with: &DateTimeRange) -> bool;
        }
    }
}

/// For the time being, an schedule is tight to a [`cycle`](Cycle).
#[derive(Debug, Clone)]
struct Schedule {
    cycle: Cycle,
    actions: HashMap<Stuff, Vec<Action>>,
}

impl Schedule {
    fn new(cycle: Cycle) -> Self {
        Schedule {
            cycle,
            actions: HashMap::new(),
        }
    }
}

#[derive(Error, Debug, Clone)]
enum ScheduleError {
    #[error("We couldn't find a valid solution for your criteria.")]
    NoValidSolutionFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_overlaps() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let dtr1 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
        );
        let dtr2 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(5, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 1).unwrap()),
        );
        let dtr3 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
        );

        assert!(dtr1.overlaps(&dtr2));
        assert!(dtr1.overlaps(&dtr3));
        assert!(dtr2.overlaps(&dtr1));
        assert!(dtr2.overlaps(&dtr3));
        assert!(dtr3.overlaps(&dtr1));
        assert!(dtr3.overlaps(&dtr2));
    }

    #[test]
    fn does_not_overlaps() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let dtr1 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
        );
        let dtr2 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
        );

        assert!(!dtr1.overlaps(&dtr2));
        assert!(!dtr2.overlaps(&dtr1)); // Since it should be reflexive.
    }

    #[test]
    fn does_contains() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let dtr1 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
        );
        let dtr2 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
        );

        assert!(dtr1.contains(&dtr2));
    }

    #[test]
    fn does_not_contains() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let dtr1 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(6, 59, 59).unwrap()),
        );
        let dtr2 = DateTimeRange::new(
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(6, 0, 0).unwrap()),
            NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
        );

        assert!(!dtr1.contains(&dtr2));
    }

    #[test]
    fn single_habit_schedule() {
        let usr_cfg = UserCfg::default();
        let cycle = Cycle {
            date_range: Range::from_duration(
                NaiveDate::from_ymd_opt(2023, 1, 23).unwrap(),
                Duration::weeks(1),
            ),
        };
        let cycles = Cycles {
            cycles: vec![cycle.clone()],
            breaks: Vec::new(),
        };

        let clean_litter = Stuff {
            id: StuffId(Uuid::new_v4()),
            kind: StuffKinds::Habit {
                estimated_duration: Duration::minutes(15),
                times: 1,
                interval: Interval::Dayly,
                schedule_options: ScheduleOpts::all_in_cycle(
                    cycles.cycles.get(0).unwrap(),
                    usr_cfg.weekly_d11,
                ),
            },
        };

        let mut schedule = Schedule::new(cycle.clone());

        let expected: Vec<_> = cycle
            .iter_days()
            .map(|date| Action {
                dt_range: DateTimeRange {
                    start: NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
                    end: NaiveDateTime::new(date, NaiveTime::from_hms_opt(7, 0, 15).unwrap()),
                },
            })
            .collect();

        println!("{:?}", schedule);

        let got: Vec<_> = schedule.actions.into_values().flatten().collect();

        assert!(expected.iter().all(|ex| got.contains(ex)));
    }

    #[test]
    #[ignore]
    fn repeatable_single_schedule() {
        let usr_cfg = UserCfg::default();
        let cycle = Cycle {
            date_range: Range::from_duration(
                NaiveDate::from_ymd_opt(2023, 1, 16).unwrap(),
                Duration::weeks(2),
            ),
        };
        let cycles = Cycles {
            cycles: vec![cycle.clone()],
            breaks: Vec::new(),
        };

        let work = Stuff {
            id: StuffId(Uuid::new_v4()),
            kind: StuffKinds::Repeatable {
                min: Some(Duration::hours(5)),
                desirable: Duration::hours(7),
                max: Some(Duration::hours(10)),
                interval: Interval::Dayly,
                schedule_options: ScheduleOpts::weekdays_in_cycle(&cycle, usr_cfg.weekly_d11),
            },
        };
    }
}
