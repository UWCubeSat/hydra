use crate::time::Time;

use super::task::Task;
use heapless::{binary_heap::Min, mpmc::MpMcQueue, BinaryHeap, LinearMap, Vec};

pub const MAX_TASKS: usize = 64;
pub const MAX_QUEUES: usize = 64;
pub const MAX_QUEUE_MESSAGES: usize = 64;

pub type Chord = Vec<Task, MAX_TASKS>;

#[macro_export]
macro_rules! chord {
    ($tasks: expr) => {{
        use $crate::scheduler::MAX_TASKS;
        use $crate::scheduler::Chord;

        let mut chord = Chord::new();
        const _: () = assert!($tasks.len() <= MAX_TASKS, "Maximum number of tasks is 64");

        for task in $tasks {
            chord.push(task).unwrap();
        }

        chord
    }};
}

type TaskHeap = BinaryHeap<Task, Min, MAX_TASKS>;
// SUPER IMPORTANT NOTE: IndexMap is broken for some reason when we try to use it here.
pub type QueueMap =
    LinearMap<&'static str, MpMcQueue<&'static str, MAX_QUEUE_MESSAGES>, MAX_QUEUES>;

pub struct Scheduler {
    tasks: TaskHeap,
    queues: QueueMap,
}

impl Scheduler {
    pub fn new(c: Chord) -> Self {
        let mut tasks: TaskHeap = TaskHeap::new();

        for t in c.into_iter() {
            tasks.push(t.clone()).unwrap();
        }

        Scheduler {
            tasks,
            queues: QueueMap::new(),
        }
    }

    /// Predicate Priority Cooperative Scheduler
    pub fn schedule(&mut self) -> ! {
        let mut next_tasks: TaskHeap = TaskHeap::new();

        loop {
            for _ in 0..self.tasks.len() {
                if let Some(mut task) = self.tasks.pop() {
                    // If our predicate fails...
                    if !task.confirm(&self.queues) {
                        next_tasks.push(task).unwrap();
                        continue;
                    }

                    task.execute(&mut self.queues);
                    task.last_ran = Time::get();
                    next_tasks.push(task).unwrap();
                    break;
                }

                // If we can't pop any off, just break.
                break;
            }

            // Rebuild Task Heap.
            for _ in 0..next_tasks.len() {
                self.tasks.push(next_tasks.pop().unwrap()).unwrap();
            }

            next_tasks.clear();
        }
    }
}
