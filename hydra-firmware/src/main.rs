#![no_main]
#![no_std]

use crate::hal::prelude::*;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use hal::pac::Peripherals;
use hydra_core::{
    scheduler::{Chord, Scheduler},
    state::State,
    task::Task,
    time::Time,
};
use panic_halt as _;
use stm32f4xx_hal as hal;

// Statically allocated Task.
const TASK_LED_TOGGLE: Task = Task::new(
    "LED Toggle",
    Task::DEFAULT_PRIORITY,
    State::EmptyState,
    |t, _| {
        // Toggles every second.
        Time::get() >= t.last_ran + 1000
    },
    |_, _| {
        // A lot of this does end up implementation specific though...
        // Different chips will want different Task Bodies.
        let p = unsafe { Peripherals::steal() };
        let gpioa = p.GPIOA.split();
        let on = gpioa.pa5.is_high();
        let mut led = gpioa.pa5.into_push_pull_output();
        if on {
            led.set_low();
        } else {
            led.set_high();
        }
    },
);

#[entry]
fn main() -> ! {
    // Enable SysTick.
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut syst = cortex_peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload((16_000) - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    // Create Chord and run it on Scheduler.
    let mut chord: Chord = Chord::new();
    chord.push(TASK_LED_TOGGLE).unwrap();
    let mut scheduler: Scheduler = Scheduler::new(chord);
    scheduler.schedule()
}

#[exception]
fn SysTick() {
    Time::increment()
}
