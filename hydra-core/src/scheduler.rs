use super::task::Task;
use heapless::{binary_heap::Min, BinaryHeap, Vec};

pub const MAX_TASKS: usize = 64;

pub type Chord = Vec<Task, MAX_TASKS>;
type TaskHeap = BinaryHeap<Task, Min, MAX_TASKS>;

pub struct Scheduler {
    tasks: TaskHeap,
}

impl Scheduler {
    pub fn new(c: Chord) -> Self {
        let mut tasks: TaskHeap = TaskHeap::new();

        for t in c.iter() {
            tasks.push(t.clone()).unwrap();
        }

        Scheduler { tasks }
    }

    /// Predicate Priority Cooperative Scheduler
    pub fn schedule(&mut self) -> ! {
        let mut next_tasks: TaskHeap = TaskHeap::new();

        loop {
            for _ in 0..self.tasks.len() {
                if let Some(mut task) = self.tasks.pop() {
                    // If our predicate fails...
                    if !task.confirm() {
                        next_tasks.push(task).unwrap();
                        continue;
                    }

                    task.execute();
                    task.last_ran += 1;
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
