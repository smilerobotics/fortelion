use crate::{error::Result, fail_status::*};

pub trait BatteryState {
    fn cell_voltages(&self) -> Result<Vec<u32>>;
    fn current(&self) -> Result<i32>;
    fn temperature(&self) -> Result<f64>;
    fn remaining_capacity(&self) -> Result<u32>;
    fn full_charge_capacity(&self) -> Result<u32>;
    fn design_capacity(&self) -> Result<u32>;
    fn absolute_state_of_charge(&self) -> Result<u32>;
    fn relative_state_of_charge(&self) -> Result<u32>;
    fn state_of_health(&self) -> Result<u32>;
    fn bm_voltage(&self) -> Result<u32>;
    fn fail_status_1(&self) -> Result<FailStatus1>;
    fn fail_status_2(&self) -> Result<FailStatus2>;
    fn fail_status_3(&self) -> Result<FailStatus3>;
}
