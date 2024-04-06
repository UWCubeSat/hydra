use heapless::mpmc::MpMcQueue;
use hydra_core::scheduler::Chord;
use hydra_core::state::State;
use hydra_core::{scheduler::Scheduler, task::Task};

fn main() {
    let task_a: Task = Task::new(
        "Task A",
        Task::DEFAULT_PRIORITY,
        State::EmptyState,
        |_, queues| {
            let msg_queue = match queues.get("msg") {
                Some(queue) => queue,
                None => return true,
            };

            msg_queue.len() == 0
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

    let task_b: Task = Task::new(
        "Task B",
        Task::DEFAULT_PRIORITY,
        State::BoolState(false),
        |_, queues| {
            let msg_queue = match queues.get("msg") {
                Some(queue) => queue,
                None => return false,
            };

            msg_queue.len() >= 1
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
            println!("Hola from {:} (Message: {:})", t.name, msg);
            if t.state == State::BoolState(false) {
                t.state = State::BoolState(true);
            } else {
                t.state = State::BoolState(false);
            }
        },
    );

    let mut c: Chord = Chord::new();
    c.push(task_a).unwrap();
    c.push(task_b).unwrap();

    let mut s: Scheduler = Scheduler::new(c);
    s.schedule()
}
