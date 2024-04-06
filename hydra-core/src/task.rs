use core::cmp::Ordering;
use core::{cmp, usize};

use crate::scheduler::QueueMap;

use super::state::State;

#[inline(always)]
pub fn true_predicate(_: &Task, _: &QueueMap) -> bool {
    true
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Task {
    pub name: &'static str,
    pub state: State,
    priority: usize,
    pub last_ran: usize,
    predicate: fn(&Task, &QueueMap) -> bool,
    func: fn(&mut Task, &mut QueueMap) -> (),
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
    pub const DEFAULT_PRIORITY: usize = usize::MAX;

    pub const fn new(
        name: &'static str,
        priority: usize,
        state: State,
        predicate: fn(&Task, &QueueMap) -> bool,
        // Tasks can mutate themselves...
        func: fn(&mut Task, &mut QueueMap),
    ) -> Self {
        Task {
            name,
            priority,
            predicate,
            func,
            state,
            last_ran: 0,
        }
    }

    /// Runs the predicate and return the given value.
    pub fn confirm(&self, queues: &QueueMap) -> bool {
        (self.predicate)(self, queues)
    }

    /// Executes the given Task!
    pub fn execute(&mut self, queues: &mut QueueMap) -> () {
        (self.func)(self, queues)
    }
}
