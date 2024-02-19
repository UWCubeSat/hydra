#![no_main]
#![no_std]

use crate::hal::prelude::*;
use cortex_m_rt::entry;
use hal::pac::Peripherals;
use hydra_core::{
    scheduler::{Chord, Scheduler},
    state::State,
    task::{true_predicate, Task},
};
use panic_halt as _;
use stm32f4xx_hal as hal;

// Statically allocated Task.
const TASK_LED_TOGGLE: Task = Task::new(
    1,
    0,
    State::BoolState(false),
    true_predicate,
    |t: &mut Task| match t.state {
        State::BoolState(led_state) => {
            let p = Peripherals::take().unwrap();
            let gpioa = p.GPIOA.split();
            let mut led = gpioa.pa5.into_push_pull_output();

            if led_state {
                led.set_low();
                t.state = State::BoolState(false);
            } else {
                led.set_high();
                t.state = State::BoolState(true);
            }
        }

        _ => {}
    },
);
#[entry]
fn main() -> ! {
    let mut chord: Chord = Chord::new();

    // Push Task onto Chord with a State.
    chord.push(TASK_LED_TOGGLE).unwrap();

    let mut scheduler: Scheduler = Scheduler::new(chord);
    scheduler.schedule()
}
