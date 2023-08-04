use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDateTime, NaiveTime, TimeZone, Utc, Weekday,
};
use std::{
    collections::HashMap,
    ops::{Deref, Range, RangeInclusive},
};
use tailcall::tailcall;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TimeUnit {
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Period(u32, TimeUnit);

struct Activity {
    id: ActivityId,
    description: String,
    goal: Option<Goal>,
    constraints: Vec<Box<dyn Constraint>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ActivityId(Uuid);

#[derive(Clone, Copy)]
struct TimeRelation {
    quantity: u32,
    unit: TimeUnit,
    period: Period,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GoalId(Uuid);

/// both `at_least` and `ideal` should be in the same time unit.
#[derive(Clone, Copy)]
struct Goal {
    id: GoalId,
    at_least: Option<TimeRelation>,
    ideal: TimeRelation,
}

trait Constraint {
    fn is_satisfied(&self, items: Vec<Scheduled>) -> bool;
}

#[derive(Clone)]
struct TimeSlot {
    range: RangeInclusive<NaiveTime>,
    weekday: Weekday,
}

impl Constraint for TimeSlot {
    fn is_satisfied(&self, items: Vec<Scheduled>) -> bool {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct MinimumSession {
    duration: Duration,
}

impl Constraint for MinimumSession {
    fn is_satisfied(&self, items: Vec<Scheduled>) -> bool {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct TimeOfDay {
    time: NaiveTime,
    duration: Duration,
}

impl Constraint for TimeOfDay {
    fn is_satisfied(&self, items: Vec<Scheduled>) -> bool {
        todo!()
    }
}

#[derive(Clone)]
struct Scheduled {
    activity_id: ActivityId,
    range: Range<DateTime<Local>>,
}

#[derive(Clone)]
struct CycleRegister<Knd> {
    start: DateTime<Utc>,
    registry: HashMap<TimeUnit, HashMap<u64, Knd>>,
}

impl<Knd> CycleRegister<Knd> {
    pub fn get_index(&self, moment: NaiveDateTime, time_unit: TimeUnit) -> u64 {
        let diff = moment - self.start.naive_utc();
        assert!(diff >= Duration::zero());

        (match time_unit {
            TimeUnit::Minutes => diff.num_minutes(),
            TimeUnit::Hours => diff.num_hours(),
            TimeUnit::Days => diff.num_days(),
            TimeUnit::Weeks => diff.num_weeks(),
            TimeUnit::Months => {
                (12 * moment.year().abs_diff(self.start.year())
                    + (moment.month() - self.start.month())) as i64
            }
            TimeUnit::Years => moment.year().abs_diff(self.start.year()) as i64,
        })
        .unsigned_abs()
    }

    pub fn get(&self, moment: NaiveDateTime, time_unit: TimeUnit) -> Option<&Knd> {
        let i = self.get_index(moment, time_unit);

        self.registry
            .get(&time_unit)
            .and_then(|periods| periods.get(&i))
    }
}

impl CycleRegister<u32> {
    pub fn add(&mut self, moment: NaiveDateTime, time_unit: TimeUnit, value: u32) {
        let i = self.get_index(moment, time_unit);

        self.registry
            .entry(time_unit)
            .or_insert_with(HashMap::new)
            .entry(i)
            .and_modify(|v| *v += value)
            .or_insert(value);
    }

    /// Assumes both `at_least` and `ideal` are in the same time unit.
    pub fn is_meeting(&self, goal: &Goal, moment: NaiveDateTime) -> bool {
        // Assuming both `at_least` and `ideal` are in the same time unit...
        let time_unit = goal.ideal.unit;

        self.get(moment, time_unit)
            .map(|value| {
                goal.at_least
                    .map(|at_least| *value >= at_least.quantity)
                    .unwrap_or(true)
                    && *value <= goal.ideal.quantity
            })
            .unwrap_or(false)
    }
}

struct Schedule {
    activities: Vec<Activity>,
    scheduled: Vec<Scheduled>,
    goal_keeper: HashMap<ActivityId, HashMap<GoalId, CycleRegister<u32>>>,
}

trait Strategy<Puzzle> {
    fn apply(&self, puzzle: Puzzle) -> Option<Puzzle>;
}

struct FixedPoint<Puzzle> {
    strategy: Box<dyn Strategy<Puzzle>>,
}

impl<Puzzle> Strategy<Puzzle> for FixedPoint<Puzzle>
where
    Puzzle: Eq + Clone,
{
    fn apply(&self, puzzle: Puzzle) -> Option<Puzzle> {
        let mut puzzle = puzzle;
        while let Some(p) = self.strategy.apply(puzzle.clone()) {
            if p == puzzle {
                break;
            } else {
                puzzle = p;
            }
        }

        Some(puzzle)
    }
}

struct Pair {
    first: Box<dyn Strategy<Schedule>>,
    second: Box<dyn Strategy<Schedule>>,
}

impl Strategy<Schedule> for Pair {
    fn apply(&self, puzzle: Schedule) -> Option<Schedule> {
        self.first.apply(puzzle).and_then(|p| self.second.apply(p))
    }
}

impl Schedule {
    pub fn is_meeting_all(&self, moment: NaiveDateTime) -> bool {
        let is_activity_met = |activity: &Activity| {
            let are_goals_met = |per_act: &HashMap<GoalId, CycleRegister<_>>| {
                activity.goal.and_then(|goal| {
                    per_act
                        .get(&goal.id)
                        .map(|cyc_reg| cyc_reg.is_meeting(&goal, moment))
                })
            };

            self.goal_keeper
                .get(&activity.id)
                .and_then(are_goals_met)
                .unwrap_or(false)
        };

        self.activities.iter().all(is_activity_met)
    }

    pub fn empty() -> Schedule {
        Schedule {
            activities: Vec::new(),
            scheduled: Vec::new(),
            goal_keeper: HashMap::new(),
        }
    }

    pub fn schedule_all(self, activities: Vec<Activity>) -> Schedule {
        activities
            .into_iter()
            .fold(Schedule::empty(), |sch, activity| sch.schedule(activity))
    }

    pub fn schedule(self, activity: Activity) -> Schedule {
        struct Env {
            activity: Activity,
            moment: NaiveDateTime,
        }

        #[tailcall]
        fn schedule_backtracking(sch: Schedule, env: Env) -> Schedule {
            if sch.is_meeting_all(env.moment) {
                return sch;
            }

            todo!()
        }

        schedule_backtracking(
            self,
            Env {
                activity,
                moment: Local::now().naive_utc(),
            },
        )
    }
}

mod tests {
    use super::*;

    #[test]
    fn schedule_single() {}
}
