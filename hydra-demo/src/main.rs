use std::thread;
use std::time::Duration;

use heapless::mpmc::MpMcQueue;
use hydra_core::chord;
use hydra_core::scheduler::Chord;
use hydra_core::state::State;
use hydra_core::time::Time;
use hydra_core::{scheduler::Scheduler, task::Task};

const TASK_A: Task = Task::new(
    "Task A",
    Task::DEFAULT_PRIORITY,
    State::EmptyState,
    |task, queues| {
        let msg_queue = match queues.get("msg") {
            Some(queue) => queue,
            None => return true,
        };

        msg_queue.is_empty() && Time::wait_until(task.last_ran, Duration::from_millis(1000))
    },
    |t, queues| {
        let msg_queue = match queues.get("msg") {
            Some(queue) => queue,
            None => {
                let _ = queues.insert("msg", MpMcQueue::new());
                queues.get("msg").unwrap()
            }
        };

        msg_queue.enqueue("This is a message!").unwrap();
        println!("Hello from {:} (Last Ran {:})", t.name, t.last_ran)
    },
);

fn main() {
    // Emulating the SysTick Timer.
    // Sets Time on 1 Millisecond Intervals.
    thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(1));
        Time::increment();
    });

    const TASK_B: Task = Task::new(
        "Task B",
        Task::DEFAULT_PRIORITY,
        State::BoolState(false),
        |_, queues| {
            let msg_queue = match queues.get("msg") {
                Some(queue) => queue,
                None => return false,
            };

            !msg_queue.is_empty()
        },
        |t, queues| {
            let msg_queue = match queues.get("msg") {
                Some(queue) => queue,
                None => {
                    let _ = queues.insert("msg", MpMcQueue::new());
                    queues.get("msg").unwrap()
                }
            };

            let msg = msg_queue.dequeue().unwrap_or("");
            println!("Hola from {:} (Message: {:}) (State: {:?})", t.name, msg, t.state);
            if let State::BoolState(state) = t.state {
                t.state = State::from(!state);
            }
        },
    );

    let c: Chord = chord!([TASK_A, TASK_B]);
    let mut s: Scheduler = Scheduler::new(c);
    s.schedule()
}
