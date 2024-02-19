use core::cmp;
use core::cmp::Ordering;

use super::state::State;

#[inline(always)]
pub fn true_predicate(_: &Task) -> bool {
    true
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Task {
    pub id: usize,
    pub state: State,
    priority: usize,
    pub last_ran: usize,
    predicate: fn(&Task) -> bool,
    func: fn(&mut Task) -> (),
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let result = self.priority.cmp(&other.priority);
        if result == Ordering::Equal {
            self.last_ran.cmp(&other.last_ran)
        } else {
            result
        }
    }
}

impl Task {
    pub const fn new(
        id: usize,
        priority: usize,
        state: State,
        predicate: fn(&Task) -> bool,
        // Tasks can mutate themselves...
        func: fn(&mut Task),
    ) -> Self {
        Task {
            id,
            priority,
            predicate,
            func,
            state,
            last_ran: 0,
        }
    }

    /// Runs the predicate and return the given value.
    pub fn confirm(&self) -> bool {
        (self.predicate)(self)
    }

    /// Executes the given Task!
    pub fn execute(&mut self) -> () {
        (self.func)(self)
    }
}
