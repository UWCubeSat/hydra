use heapless::spsc::Queue;
use hydra_core::scheduler::Chord;
use hydra_core::state::State;
use hydra_core::{
    scheduler::Scheduler,
    task::Task,
};


// This is currently how we do shared variables...
// There is likely a way to do this within the Scheduler.
const MAX_MSG_LENGTH: usize = 128;
static mut MSG_QUEUE: Queue<&str, MAX_MSG_LENGTH> = Queue::new();

// Tasks are Compile Time Known
const TASK_A: Task = Task::new(
    "Task A",
    Task::DEFAULT_PRIORITY,
    State::EmptyState,
    |_t: &Task| unsafe { !MSG_QUEUE.is_full() },
    |t: &mut Task| {
        unsafe {
            MSG_QUEUE.enqueue("ABCDEF").unwrap();
        }
        println!("Hello from {:} (Last Ran {:})", t.name, t.last_ran)
    },
);

// Tasks can mutate themselves at run time.
const TASK_B: Task = Task::new(
    "Task B",
    Task::DEFAULT_PRIORITY,
    State::BoolState(false),
    |_| unsafe { MSG_QUEUE.len() > 0 },
    |t| {
        let msg: &str = unsafe { MSG_QUEUE.dequeue().unwrap() };
        println!("Hola from {:} (Message: {:})", t.name, msg);
        if t.state == State::BoolState(false) {
            t.state = State::BoolState(true);
        } else {
            t.state = State::BoolState(false);
        }
    },
);

fn main() {
    let mut c: Chord = Chord::new();

    c.push(TASK_A).unwrap();
    c.push(TASK_B).unwrap();

    let mut s: Scheduler = Scheduler::new(c);
    s.schedule()
}
