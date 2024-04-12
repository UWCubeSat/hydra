static mut HYDRA_TIME: u64 = 0;

pub struct Time {}

impl Time {
    pub fn increment() -> () {
        unsafe { HYDRA_TIME = HYDRA_TIME.wrapping_add(1) };
    }

    pub fn get() -> u64 {
        unsafe { HYDRA_TIME }
    }
}
