use clap::Parser;
use hydra_core::scheduler::Chord;
use hydra_core::state::State;
use hydra_core::{
    scheduler::Scheduler,
    task::{true_predicate, Task},
};

#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value_t = 1)]
    tasks: usize,
    #[arg(short, long, default_value_t = 5)]
    nodes: usize,
}

// Tasks are Compile Time Known (prefered!)
const TASK_A: Task = Task::new(0, 0, State::EmptyState, true_predicate, |t: &mut Task| {
    println!("Hello from Task #{:} (Last Ran {:})", t.id, t.last_ran)
});

fn main() {
    // let config = Config::parse();
    let mut c: Chord = Chord::new();
    c.push(TASK_A).unwrap();

    let mut s: Scheduler = Scheduler::new(c);
    s.schedule()
}
