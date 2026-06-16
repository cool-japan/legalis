//! Deadline tracking and reminders.
//!
//! [`BusinessCalendar`] provides real business-day arithmetic (configurable
//! weekend days and holidays, with a generator for U.S. federal holidays). On
//! top of it, [`DeadlineTracker`] stores [`Deadline`]s, computes their
//! [`DeadlineStatus`] relative to a reference date, schedules deadlines a number
//! of *business* days out, and turns per-deadline reminder offsets into a
//! concrete list of due [`Reminder`]s.

use super::Criticality;
use crate::Jurisdiction;
use anyhow::{Result, anyhow};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// ============================================================================
// Business calendar
// ============================================================================

/// A configurable calendar for business-day arithmetic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessCalendar {
    /// Days of the week treated as non-working.
    weekend: HashSet<Weekday>,
    /// Specific dates treated as holidays.
    holidays: HashSet<NaiveDate>,
}

impl Default for BusinessCalendar {
    fn default() -> Self {
        let mut weekend = HashSet::new();
        weekend.insert(Weekday::Sat);
        weekend.insert(Weekday::Sun);
        Self {
            weekend,
            holidays: HashSet::new(),
        }
    }
}

impl BusinessCalendar {
    /// Creates a calendar with a Saturday/Sunday weekend and no holidays.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the set of weekend days.
    pub fn with_weekend<I: IntoIterator<Item = Weekday>>(mut self, days: I) -> Self {
        self.weekend = days.into_iter().collect();
        self
    }

    /// Adds a holiday (builder style).
    pub fn with_holiday(mut self, date: NaiveDate) -> Self {
        self.holidays.insert(date);
        self
    }

    /// Adds a holiday.
    pub fn add_holiday(&mut self, date: NaiveDate) {
        self.holidays.insert(date);
    }

    /// Adds all U.S. federal holidays for the given year.
    pub fn with_us_federal_holidays(mut self, year: i32) -> Self {
        for date in Self::us_federal_holidays(year) {
            self.holidays.insert(date);
        }
        self
    }

    /// Returns whether the date falls on a weekend.
    pub fn is_weekend(&self, date: NaiveDate) -> bool {
        self.weekend.contains(&date.weekday())
    }

    /// Returns whether the date is a registered holiday.
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holidays.contains(&date)
    }

    /// Returns whether the date is a working business day.
    pub fn is_business_day(&self, date: NaiveDate) -> bool {
        !self.is_weekend(date) && !self.is_holiday(date)
    }

    /// Returns the next business day strictly after `date`.
    pub fn next_business_day(&self, date: NaiveDate) -> NaiveDate {
        let mut current = date;
        while let Some(next) = current.succ_opt() {
            current = next;
            if self.is_business_day(current) {
                return current;
            }
        }
        current
    }

    /// Returns the previous business day strictly before `date`.
    pub fn previous_business_day(&self, date: NaiveDate) -> NaiveDate {
        let mut current = date;
        while let Some(prev) = current.pred_opt() {
            current = prev;
            if self.is_business_day(current) {
                return current;
            }
        }
        current
    }

    /// Returns the date `count` business days after `start`.
    ///
    /// A non-positive `count` returns `start` unchanged.
    pub fn add_business_days(&self, start: NaiveDate, count: i64) -> NaiveDate {
        if count <= 0 {
            return start;
        }
        let mut current = start;
        for _ in 0..count {
            current = self.next_business_day(current);
        }
        current
    }

    /// Counts business days in the half-open interval `(from, to]`.
    ///
    /// The result is negative when `to` precedes `from`.
    pub fn business_days_between(&self, from: NaiveDate, to: NaiveDate) -> i64 {
        if from == to {
            return 0;
        }
        let (start, end, sign) = if to > from {
            (from, to, 1)
        } else {
            (to, from, -1)
        };
        let mut count = 0i64;
        let mut current = start;
        while let Some(next) = current.succ_opt() {
            current = next;
            if current > end {
                break;
            }
            if self.is_business_day(current) {
                count += 1;
            }
        }
        sign * count
    }

    /// Computes the U.S. federal holidays observed in a given year.
    pub fn us_federal_holidays(year: i32) -> Vec<NaiveDate> {
        let mut days = Vec::new();
        let mut push = |opt: Option<NaiveDate>| {
            if let Some(date) = opt {
                days.push(date);
            }
        };
        push(NaiveDate::from_ymd_opt(year, 1, 1)); // New Year's Day
        push(nth_weekday(year, 1, Weekday::Mon, 3)); // MLK Day
        push(nth_weekday(year, 2, Weekday::Mon, 3)); // Presidents' Day
        push(last_weekday(year, 5, Weekday::Mon)); // Memorial Day
        push(NaiveDate::from_ymd_opt(year, 6, 19)); // Juneteenth
        push(NaiveDate::from_ymd_opt(year, 7, 4)); // Independence Day
        push(nth_weekday(year, 9, Weekday::Mon, 1)); // Labor Day
        push(nth_weekday(year, 10, Weekday::Mon, 2)); // Columbus Day
        push(NaiveDate::from_ymd_opt(year, 11, 11)); // Veterans Day
        push(nth_weekday(year, 11, Weekday::Thu, 4)); // Thanksgiving
        push(NaiveDate::from_ymd_opt(year, 12, 25)); // Christmas Day
        days
    }
}

/// Returns the `n`-th occurrence (1-based) of `weekday` in a month.
fn nth_weekday(year: i32, month: u32, weekday: Weekday, n: u32) -> Option<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_dow = first.weekday().num_days_from_monday();
    let target = weekday.num_days_from_monday();
    let offset = (7 + target - first_dow) % 7;
    let day = 1 + offset + (n.saturating_sub(1)) * 7;
    NaiveDate::from_ymd_opt(year, month, day)
}

/// Returns the last occurrence of `weekday` in a month.
fn last_weekday(year: i32, month: u32, weekday: Weekday) -> Option<NaiveDate> {
    let mut current = last_day_of_month(year, month)?;
    while current.weekday() != weekday {
        current = current.pred_opt()?;
    }
    Some(current)
}

/// Returns the last calendar day of a month.
fn last_day_of_month(year: i32, month: u32) -> Option<NaiveDate> {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd_opt(next_year, next_month, 1)?.pred_opt()
}

// ============================================================================
// Deadlines
// ============================================================================

/// The status of a deadline relative to a reference date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeadlineStatus {
    /// Already completed.
    Completed,
    /// Past due and not completed.
    Overdue,
    /// Due on the reference date.
    DueToday,
    /// Due within the "due soon" window.
    DueSoon,
    /// Further out than the "due soon" window.
    Upcoming,
}

impl DeadlineStatus {
    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            DeadlineStatus::Completed => "completed",
            DeadlineStatus::Overdue => "overdue",
            DeadlineStatus::DueToday => "due today",
            DeadlineStatus::DueSoon => "due soon",
            DeadlineStatus::Upcoming => "upcoming",
        }
    }

    /// Returns whether the status requires attention (overdue or due soon).
    pub fn needs_attention(&self) -> bool {
        matches!(
            self,
            DeadlineStatus::Overdue | DeadlineStatus::DueToday | DeadlineStatus::DueSoon
        )
    }
}

/// A tracked legal deadline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deadline {
    /// Stable identifier.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// The due date.
    pub due_date: NaiveDate,
    /// Grouping (e.g. `filing`, `discovery`, `renewal`).
    pub category: String,
    /// Optional jurisdiction.
    pub jurisdiction: Option<Jurisdiction>,
    /// Priority band.
    pub priority: Criticality,
    /// Whether the deadline has been met.
    pub completed: bool,
    /// Reminder lead times, in calendar days before the due date.
    pub reminder_offsets: Vec<i64>,
}

impl Deadline {
    /// Creates a new deadline (default category `general`, `Medium` priority).
    pub fn new(id: impl Into<String>, title: impl Into<String>, due_date: NaiveDate) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            due_date,
            category: "general".to_string(),
            jurisdiction: None,
            priority: Criticality::Medium,
            completed: false,
            reminder_offsets: Vec::new(),
        }
    }

    /// Sets the category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// Sets the priority.
    pub fn with_priority(mut self, priority: Criticality) -> Self {
        self.priority = priority;
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Sets the reminder lead times (calendar days before due).
    pub fn with_reminder_offsets<I: IntoIterator<Item = i64>>(mut self, offsets: I) -> Self {
        let mut offsets: Vec<i64> = offsets.into_iter().filter(|days| *days >= 0).collect();
        offsets.sort_unstable_by(|a, b| b.cmp(a));
        offsets.dedup();
        self.reminder_offsets = offsets;
        self
    }

    /// Marks the deadline as completed.
    pub fn mark_complete(&mut self) {
        self.completed = true;
    }

    /// Returns the number of days until the deadline (negative if overdue).
    pub fn days_until(&self, today: NaiveDate) -> i64 {
        (self.due_date - today).num_days()
    }

    /// Returns whether the deadline is past due and not completed.
    pub fn is_overdue(&self, today: NaiveDate) -> bool {
        !self.completed && self.due_date < today
    }

    /// Computes the deadline status relative to `today`.
    pub fn status(&self, today: NaiveDate, due_soon_window: i64) -> DeadlineStatus {
        if self.completed {
            return DeadlineStatus::Completed;
        }
        let days = self.days_until(today);
        if days < 0 {
            DeadlineStatus::Overdue
        } else if days == 0 {
            DeadlineStatus::DueToday
        } else if days <= due_soon_window {
            DeadlineStatus::DueSoon
        } else {
            DeadlineStatus::Upcoming
        }
    }
}

/// A concrete reminder that is due to fire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    /// Id of the originating deadline.
    pub deadline_id: String,
    /// Title of the originating deadline.
    pub deadline_title: String,
    /// The deadline's due date.
    pub due_date: NaiveDate,
    /// The date this reminder should fire.
    pub fire_date: NaiveDate,
    /// Lead time in days this reminder represents.
    pub lead_days: i64,
    /// Pre-rendered reminder message.
    pub message: String,
}

// ============================================================================
// Tracker
// ============================================================================

/// Tracks deadlines and produces statuses and reminders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineTracker {
    deadlines: Vec<Deadline>,
    calendar: BusinessCalendar,
    due_soon_window: i64,
}

impl Default for DeadlineTracker {
    fn default() -> Self {
        Self {
            deadlines: Vec::new(),
            calendar: BusinessCalendar::default(),
            due_soon_window: 7,
        }
    }
}

impl DeadlineTracker {
    /// Creates a tracker with a default calendar and a 7-day "due soon" window.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the business calendar.
    pub fn with_calendar(mut self, calendar: BusinessCalendar) -> Self {
        self.calendar = calendar;
        self
    }

    /// Sets the "due soon" window (in days).
    pub fn with_due_soon_window(mut self, days: i64) -> Self {
        self.due_soon_window = days.max(0);
        self
    }

    /// Returns the business calendar.
    pub fn calendar(&self) -> &BusinessCalendar {
        &self.calendar
    }

    /// Adds a deadline.
    pub fn add(&mut self, deadline: Deadline) {
        self.deadlines.push(deadline);
    }

    /// Returns a deadline by id.
    pub fn get(&self, id: &str) -> Option<&Deadline> {
        self.deadlines.iter().find(|deadline| deadline.id == id)
    }

    /// Returns the number of tracked deadlines.
    pub fn len(&self) -> usize {
        self.deadlines.len()
    }

    /// Returns whether no deadlines are tracked.
    pub fn is_empty(&self) -> bool {
        self.deadlines.is_empty()
    }

    /// Marks a deadline complete, returning an error for an unknown id.
    pub fn complete(&mut self, id: &str) -> Result<()> {
        let deadline = self
            .deadlines
            .iter_mut()
            .find(|deadline| deadline.id == id)
            .ok_or_else(|| anyhow!("unknown deadline: {}", id))?;
        deadline.mark_complete();
        Ok(())
    }

    /// Schedules a deadline a number of *business* days after `start`, adds it
    /// to the tracker and returns a copy.
    pub fn schedule_business(
        &mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        start: NaiveDate,
        business_days: i64,
    ) -> Deadline {
        let due_date = self.calendar.add_business_days(start, business_days);
        let deadline = Deadline::new(id, title, due_date);
        self.deadlines.push(deadline.clone());
        deadline
    }

    /// Returns the status of every deadline relative to `today`.
    pub fn statuses(&self, today: NaiveDate) -> Vec<(String, DeadlineStatus)> {
        self.deadlines
            .iter()
            .map(|deadline| {
                (
                    deadline.id.clone(),
                    deadline.status(today, self.due_soon_window),
                )
            })
            .collect()
    }

    /// Returns overdue deadlines, soonest first.
    pub fn overdue(&self, today: NaiveDate) -> Vec<&Deadline> {
        let mut overdue: Vec<&Deadline> = self
            .deadlines
            .iter()
            .filter(|deadline| deadline.is_overdue(today))
            .collect();
        overdue.sort_by_key(|deadline| deadline.due_date);
        overdue
    }

    /// Returns incomplete deadlines due within `days` of `today` (inclusive).
    pub fn due_within(&self, today: NaiveDate, days: i64) -> Vec<&Deadline> {
        let mut due: Vec<&Deadline> = self
            .deadlines
            .iter()
            .filter(|deadline| {
                if deadline.completed {
                    return false;
                }
                let remaining = deadline.days_until(today);
                (0..=days).contains(&remaining)
            })
            .collect();
        due.sort_by_key(|deadline| deadline.due_date);
        due
    }

    /// Returns incomplete deadlines due on or after `today`, soonest first.
    pub fn upcoming(&self, today: NaiveDate) -> Vec<&Deadline> {
        let mut upcoming: Vec<&Deadline> = self
            .deadlines
            .iter()
            .filter(|deadline| !deadline.completed && deadline.due_date >= today)
            .collect();
        upcoming.sort_by_key(|deadline| deadline.due_date);
        upcoming
    }

    /// Returns the next incomplete deadline due on or after `today`.
    pub fn next_deadline(&self, today: NaiveDate) -> Option<&Deadline> {
        self.upcoming(today).into_iter().next()
    }

    /// Returns reminders that are due to fire on or before `today`.
    ///
    /// For each incomplete deadline and each reminder offset, the fire date is
    /// `due_date - offset` (calendar days); a reminder is returned when its fire
    /// date has arrived and the deadline has not yet passed.
    pub fn pending_reminders(&self, today: NaiveDate) -> Vec<Reminder> {
        let mut reminders = Vec::new();
        for deadline in &self.deadlines {
            if deadline.completed {
                continue;
            }
            for &offset in &deadline.reminder_offsets {
                let fire_date = match deadline.due_date.checked_sub_signed(Duration::days(offset)) {
                    Some(date) => date,
                    None => continue,
                };
                if fire_date <= today && today <= deadline.due_date {
                    let remaining = deadline.days_until(today);
                    let message = if remaining == 0 {
                        format!("'{}' is due today", deadline.title)
                    } else {
                        format!("'{}' is due in {} day(s)", deadline.title, remaining)
                    };
                    reminders.push(Reminder {
                        deadline_id: deadline.id.clone(),
                        deadline_title: deadline.title.clone(),
                        due_date: deadline.due_date,
                        fire_date,
                        lead_days: offset,
                        message,
                    });
                }
            }
        }
        reminders.sort_by(|a, b| {
            a.fire_date
                .cmp(&b.fire_date)
                .then(a.deadline_id.cmp(&b.deadline_id))
                .then(b.lead_days.cmp(&a.lead_days))
        });
        reminders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
    }

    #[test]
    fn test_business_day_arithmetic() {
        let calendar = BusinessCalendar::new();
        let friday = date(2026, 6, 12);
        assert!(calendar.is_business_day(friday));
        assert!(calendar.is_weekend(date(2026, 6, 13))); // Saturday
        // One business day after Friday is Monday.
        assert_eq!(calendar.add_business_days(friday, 1), date(2026, 6, 15));
        assert_eq!(calendar.next_business_day(friday), date(2026, 6, 15));
        assert_eq!(
            calendar.previous_business_day(date(2026, 6, 15)),
            date(2026, 6, 12)
        );
        // Mon..Fri is 4 business days after start (excluding start).
        assert_eq!(
            calendar.business_days_between(date(2026, 6, 15), date(2026, 6, 19)),
            4
        );
    }

    #[test]
    fn test_holiday_handling() {
        let calendar = BusinessCalendar::new().with_us_federal_holidays(2026);
        let holidays = BusinessCalendar::us_federal_holidays(2026);
        assert_eq!(holidays.len(), 11);
        assert!(holidays.contains(&date(2026, 7, 4))); // Independence Day
        assert!(holidays.contains(&date(2026, 1, 19))); // MLK Day (3rd Mon Jan)
        assert!(holidays.contains(&date(2026, 5, 25))); // Memorial Day (last Mon May)
        // Thanksgiving 2026 is the 4th Thursday of November (Nov 26, a Thursday).
        let thanksgiving = date(2026, 11, 26);
        assert!(holidays.contains(&thanksgiving));
        assert!(!calendar.is_business_day(thanksgiving));
        // Crossing Thanksgiving skips it: Wed Nov 25 + 1 business day -> Fri Nov 27.
        assert_eq!(
            calendar.add_business_days(date(2026, 11, 25), 1),
            date(2026, 11, 27)
        );
    }

    #[test]
    fn test_deadline_status() {
        let today = date(2026, 6, 14);
        let mut overdue = Deadline::new("d1", "File answer", date(2026, 6, 10));
        assert_eq!(overdue.status(today, 7), DeadlineStatus::Overdue);
        assert!(overdue.is_overdue(today));
        overdue.mark_complete();
        assert_eq!(overdue.status(today, 7), DeadlineStatus::Completed);

        let due_today = Deadline::new("d2", "Hearing", today);
        assert_eq!(due_today.status(today, 7), DeadlineStatus::DueToday);

        let due_soon = Deadline::new("d3", "Discovery", date(2026, 6, 18));
        assert_eq!(due_soon.status(today, 7), DeadlineStatus::DueSoon);

        let upcoming = Deadline::new("d4", "Trial", date(2026, 8, 1));
        assert_eq!(upcoming.status(today, 7), DeadlineStatus::Upcoming);
    }

    #[test]
    fn test_schedule_business_and_queries() {
        let mut tracker = DeadlineTracker::new();
        let start = date(2026, 6, 12); // Friday
        let deadline = tracker.schedule_business("resp", "Response", start, 5);
        // 5 business days after Friday -> following Friday (Jun 19).
        assert_eq!(deadline.due_date, date(2026, 6, 19));
        assert_eq!(tracker.len(), 1);

        let today = date(2026, 6, 14);
        assert_eq!(tracker.upcoming(today).len(), 1);
        assert_eq!(
            tracker.next_deadline(today).map(|d| d.id.as_str()),
            Some("resp")
        );
        assert!(tracker.overdue(today).is_empty());
    }

    #[test]
    fn test_pending_reminders() {
        let mut tracker = DeadlineTracker::new();
        tracker.add(
            Deadline::new("d1", "File brief", date(2026, 6, 20)).with_reminder_offsets([14, 7, 1]),
        );
        // 7 days before due (Jun 13) -> on Jun 14 the 14-day and 7-day reminders fire.
        let reminders = tracker.pending_reminders(date(2026, 6, 14));
        assert_eq!(reminders.len(), 2);
        assert!(reminders.iter().all(|r| r.deadline_id == "d1"));
        assert!(reminders[0].message.contains("due in"));

        // After completion, no reminders.
        tracker.complete("d1").expect("ok");
        assert!(tracker.pending_reminders(date(2026, 6, 14)).is_empty());
        assert!(tracker.complete("missing").is_err());
    }
}
