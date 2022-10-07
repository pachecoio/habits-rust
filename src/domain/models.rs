use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Habit {
	pub name: String,
	pub schedule: Schedule
}

impl Habit {
	pub fn new(name: &str, schedule: &Schedule) -> Self {
		Self {
			name: name.to_string(),
			schedule: schedule.clone(),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidWeekdays {}

impl fmt::Display for InvalidWeekdays {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid weekdays")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule {
	frequency: i32,
	days: Vec<i32>
}

impl Schedule {
    pub fn new(frequency: i32, days: &[i32]) -> Result<Self, InvalidWeekdays> {
		for day in days {
			if *day < 1 || *day > 7 {
				return Err(InvalidWeekdays{})
			}
		}
        Ok(Self {
			frequency,
			days: Vec::from(days)
		})
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
	pub name: String,
	pub last_name: String
}

impl User {
	pub fn new(name: &str, last_name: &str) -> Self {
        Self {
            name: name.to_string(),
            last_name: last_name.to_string()
        }
    }

	pub fn fullname(&self) -> String {
		format!("{} {}", self.name, self.last_name)
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HabitLogStatus {
	Pending,
	Completed,
	Skipped,
}

pub struct HabitEvent {
    pub user: User,
    pub habit: Habit,
	pub status: HabitLogStatus
}

impl HabitEvent {
	pub fn new(user: &User, habit: &Habit) -> Self {
        Self {
            user: user.clone(),
            habit: habit.clone(),
			status: HabitLogStatus::Pending
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidHabitStatusChange {}

impl fmt::Display for InvalidHabitStatusChange {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid status change")
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_habit_instance() {
		let expected_schedule = unsafe {Schedule::new(
			1,
            &[1, 2, 3, 4, 5, 6, 7]
		).unwrap_unchecked()};
		let habit = Habit::new("Read 10 pages", &expected_schedule);
        assert_eq!(habit.name, "Read 10 pages");
		assert_eq!(habit.schedule, expected_schedule);
	}

	#[test]
	fn test_cannot_create_schedule_with_invalid_weekdays() {
		let error = Schedule::new(
            1,
            &[8]
        ).unwrap_err();
		assert_eq!(error, InvalidWeekdays {})
	}

	#[test]
    fn test_user_instance() {
		let user = User::new("Thiago", "Pacheco");
		assert_eq!(user.fullname(), "Thiago Pacheco");
	}

	#[test]
    fn test_habit_event() {
		let user = User::new("Thiago", "Pacheco");
		let schedule = unsafe {Schedule::new(1, &[1, 2, 3, 4]).unwrap_unchecked()};
		let read_habit = Habit::new("Read 10 pages", &schedule);

		let habit_event = HabitEvent::new(&user, &read_habit);
		assert_eq!(habit_event.user.fullname(), "Thiago Pacheco");
		assert_eq!(habit_event.habit.name, "Read 10 pages");
		assert_eq!(habit_event.status, HabitLogStatus::Pending);
	}

}