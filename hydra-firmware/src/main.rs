#![no_main]
#![no_std]

use crate::hal::prelude::*;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use hal::pac::Peripherals;
use hydra_core::{scheduler::{Chord, Scheduler}, state::State, task::{true_predicate, Task}};
use panic_halt as _;
use stm32f4xx_hal as hal;

// Statically allocated Task.
const TASK_LED_TOGGLE: Task = Task::new(
    "LED Toggle",
    0,
    State::BoolState(false),
    true_predicate,
    |t: &mut Task, _| match t.state {
        State::BoolState(led_state) => {
            let p = unsafe { Peripherals::steal() };
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
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

    // We need to pass Peripherals down to the Scheduler.
    let mut syst = cortex_peripherals.SYST;

    // Enable SysTick.
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload((16_000) - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    let mut chord: Chord = Chord::new();

    // Push Task onto Chord with a State.
    chord.push(TASK_LED_TOGGLE).unwrap();

    let mut scheduler: Scheduler = Scheduler::new(chord);
    scheduler.schedule()
}

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;

    *COUNT += 1;
}
