use core::time::Duration;

static mut HYDRA_TIME: u128 = 0;

pub struct Time {}

impl Time {
    pub fn increment() -> () {
        unsafe { HYDRA_TIME = HYDRA_TIME.wrapping_add(1) };
    }

    pub fn get() -> u128 {
        unsafe { HYDRA_TIME }
    }

    pub fn wait_until(last_ran: u128, wait: Duration) -> bool {
        unsafe { HYDRA_TIME >= last_ran + wait.as_millis() }
    }
}
