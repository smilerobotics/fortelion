use strum::{Display, EnumIter, IntoEnumIterator};

use crate::BatteryState;

#[derive(Debug, Default, PartialEq)]
pub enum FailState {
    Ok,
    Ng,
    #[default]
    Unknown,
}

pub struct FailStatus1(pub(crate) u8);
pub struct FailStatus2(pub(crate) u8);
pub struct FailStatus3(pub(crate) u8);

const FAIL_STATUS_1_OVER_CURRENT_DISCHARGE_DETECTION_65A_BIT: usize = 0;
const FAIL_STATUS_1_OVER_CURRENT_DISCHARGE_DETECTION_90A_BIT: usize = 1;
const FAIL_STATUS_1_OVER_CHARGE_PROTECTION_BIT: usize = 2;
const FAIL_STATUS_1_OVER_CURRENT_CHARGE_DETECTION_45A_BIT: usize = 3;
const FAIL_STATUS_1_OVER_TEMPERATURE_DISCHARGE_DETECTION_BIT: usize = 4;
const FAIL_STATUS_1_LOW_VOLTAGE_DETECTION_BIT: usize = 5;
const FAIL_STATUS_1_FULLY_CHARGE_DETECTION_BIT: usize = 6;
const FAIL_STATUS_1_OVER_CURRENT_DISCHARGE_DETECTION_200A_BIT: usize = 7;
const FAIL_STATUS_2_OVER_CURRENT_DISCHARGE_DETECTION_110A_BIT: usize = 0;
const FAIL_STATUS_2_OVER_CURRENT_CHARGE_DETECTION_65A_BIT: usize = 1;
const FAIL_STATUS_2_OVER_TEMPERATURE_CHARGE_DETECTION_BIT: usize = 2;
const FAIL_STATUS_2_CELL_UNBALANCE_DETECTION_BIT: usize = 3;
const FAIL_STATUS_2_OVER_CHARGE_BIT: usize = 4;
const FAIL_STATUS_2_DEEP_DISCHARGE_BIT: usize = 5;
const FAIL_STATUS_2_FUSE_BLOWN_BIT: usize = 6;
const FAIL_STATUS_2_FET_UNCONTROL_BIT: usize = 7;
const FAIL_STATUS_3_SELF_TEST_CLOCK_FAIL_BIT: usize = 0;
const FAIL_STATUS_3_SELF_TEST_ROM_FAIL_BIT: usize = 1;
const FAIL_STATUS_3_SELF_TEST_REGISTER_FAIL_BIT: usize = 2;
const FAIL_STATUS_3_SELF_TEST_PSW_REGISTER_FAIL_BIT: usize = 3;
const FAIL_STATUS_3_SELF_TEST_STACK_REGISTER_FAIL_BIT: usize = 4;
const FAIL_STATUS_3_SELF_TEST_CS_REGISTER_FAIL_BIT: usize = 5;
const FAIL_STATUS_3_SELF_TEST_ES_REGISTER_FAIL_BIT: usize = 6;
const FAIL_STATUS_3_SELF_TEST_RAM_FAIL_DF_FAIL_BIT: usize = 7;

fn fail_state_from_byte_and_bit_pos(byte: u8, pos: usize) -> FailState {
    if byte & (0x01 << pos) == 0 {
        FailState::Ok
    } else {
        FailState::Ng
    }
}

impl FailStatus1 {
    pub fn over_current_discharge_detection_65a(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_1_OVER_CURRENT_DISCHARGE_DETECTION_65A_BIT,
        )
    }

    pub fn over_current_discharge_detection_90a(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_1_OVER_CURRENT_DISCHARGE_DETECTION_90A_BIT,
        )
    }

    pub fn over_charge_protection(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_1_OVER_CHARGE_PROTECTION_BIT)
    }

    pub fn over_current_charge_detection_45a(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_1_OVER_CURRENT_CHARGE_DETECTION_45A_BIT,
        )
    }

    pub fn over_temperature_discharge_detection(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_1_OVER_TEMPERATURE_DISCHARGE_DETECTION_BIT,
        )
    }

    pub fn low_voltage_detection(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_1_LOW_VOLTAGE_DETECTION_BIT)
    }

    pub fn fully_charge_detection(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_1_FULLY_CHARGE_DETECTION_BIT)
    }

    pub fn over_current_discharge_detection_200a(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_1_OVER_CURRENT_DISCHARGE_DETECTION_200A_BIT,
        )
    }
}

impl FailStatus2 {
    pub fn over_current_discharge_detection_110a(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_2_OVER_CURRENT_DISCHARGE_DETECTION_110A_BIT,
        )
    }

    pub fn over_current_charge_detection_65a(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_2_OVER_CURRENT_CHARGE_DETECTION_65A_BIT,
        )
    }

    pub fn over_temperature_charge_detection(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(
            self.0,
            FAIL_STATUS_2_OVER_TEMPERATURE_CHARGE_DETECTION_BIT,
        )
    }

    pub fn cell_unbalance_detection(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_2_CELL_UNBALANCE_DETECTION_BIT)
    }

    pub fn over_charge(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_2_OVER_CHARGE_BIT)
    }

    pub fn deep_discharge(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_2_DEEP_DISCHARGE_BIT)
    }

    pub fn fuse_blown(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_2_FUSE_BLOWN_BIT)
    }

    pub fn fet_uncontrol(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_2_FET_UNCONTROL_BIT)
    }
}

impl FailStatus3 {
    pub fn self_test_clock_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_CLOCK_FAIL_BIT)
    }

    pub fn self_test_rom_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_ROM_FAIL_BIT)
    }

    pub fn self_test_register_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_REGISTER_FAIL_BIT)
    }

    pub fn self_test_psw_register_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_PSW_REGISTER_FAIL_BIT)
    }

    pub fn self_test_stack_register_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_STACK_REGISTER_FAIL_BIT)
    }

    pub fn self_test_cs_register_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_CS_REGISTER_FAIL_BIT)
    }

    pub fn self_test_es_register_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_ES_REGISTER_FAIL_BIT)
    }

    pub fn self_test_ram_fail_df_fail(&self) -> FailState {
        fail_state_from_byte_and_bit_pos(self.0, FAIL_STATUS_3_SELF_TEST_RAM_FAIL_DF_FAIL_BIT)
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, PartialEq)]
pub enum FailStatusItem {
    OverCurrentDischargeDetection65A,
    OverCurrentDischargeDetection90A,
    OverChargeProtection,
    OverCurrentChargeDetection45A,
    OverTemperatureDischargeDetection,
    LowVoltageDetection,
    FullyChargeDetection,
    OverCurrentDischargeDetection200A,
    OverCurrentDischargeDetection110A,
    OverCurrentChargeDetection65A,
    OverTemperatureChargeDetection,
    CellUnbalanceDetection,
    OverCharge,
    DeepDischarge,
    FuseBlown,
    FetUncontrol,
    SelfTestClockFail,
    SelfTestRomFail,
    SelfTestRegisterFail,
    SelfTestPswRegisterFail,
    SelfTestStackRegisterFail,
    SelfTestCsRegisterFail,
    SelfTestEsRegisterFail,
    SelfTestRamFailDfFail,
}

pub struct FailStatusValuesIter<'a, S>
where
    S: FailStatus + ?Sized,
{
    fail_status: &'a S,
    items_iter: FailStatusItemIter,
}

impl<'a, S> FailStatusValuesIter<'a, S>
where
    S: FailStatus + ?Sized,
{
    pub fn new(fail_status: &'a S) -> Self {
        let items_iter = FailStatusItem::iter();
        Self {
            fail_status,
            items_iter,
        }
    }
}

impl<'a, S> Iterator for FailStatusValuesIter<'a, S>
where
    S: FailStatus + ?Sized,
{
    type Item = (FailStatusItem, FailState);

    fn next(&mut self) -> Option<Self::Item> {
        self.items_iter
            .next()
            .map(|item| (item, self.fail_status.fail_status(item)))
    }
}

pub trait FailStatus {
    fn fail_status(&self, item: FailStatusItem) -> FailState;

    fn fail_status_values(&self) -> FailStatusValuesIter<Self> {
        FailStatusValuesIter::new(self)
    }
}

impl<B: BatteryState> FailStatus for B {
    fn fail_status(&self, item: FailStatusItem) -> FailState {
        use FailStatusItem::*;
        match item {
            OverCurrentDischargeDetection65A => self
                .fail_status_1()
                .map(|status| status.over_current_discharge_detection_65a())
                .unwrap_or_default(),
            OverCurrentDischargeDetection90A => self
                .fail_status_1()
                .map(|status| status.over_current_discharge_detection_90a())
                .unwrap_or_default(),
            OverChargeProtection => self
                .fail_status_1()
                .map(|status| status.over_charge_protection())
                .unwrap_or_default(),
            OverCurrentChargeDetection45A => self
                .fail_status_1()
                .map(|status| status.over_current_charge_detection_45a())
                .unwrap_or_default(),
            OverTemperatureDischargeDetection => self
                .fail_status_1()
                .map(|status| status.over_temperature_discharge_detection())
                .unwrap_or_default(),
            LowVoltageDetection => self
                .fail_status_1()
                .map(|status| status.low_voltage_detection())
                .unwrap_or_default(),
            FullyChargeDetection => self
                .fail_status_1()
                .map(|status| status.fully_charge_detection())
                .unwrap_or_default(),
            OverCurrentDischargeDetection200A => self
                .fail_status_1()
                .map(|status| status.over_current_discharge_detection_200a())
                .unwrap_or_default(),
            OverCurrentDischargeDetection110A => self
                .fail_status_2()
                .map(|status| status.over_current_discharge_detection_110a())
                .unwrap_or_default(),
            OverCurrentChargeDetection65A => self
                .fail_status_2()
                .map(|status| status.over_current_charge_detection_65a())
                .unwrap_or_default(),
            OverTemperatureChargeDetection => self
                .fail_status_2()
                .map(|status| status.over_temperature_charge_detection())
                .unwrap_or_default(),
            CellUnbalanceDetection => self
                .fail_status_2()
                .map(|status| status.cell_unbalance_detection())
                .unwrap_or_default(),
            OverCharge => self
                .fail_status_2()
                .map(|status| status.over_charge())
                .unwrap_or_default(),
            DeepDischarge => self
                .fail_status_2()
                .map(|status| status.deep_discharge())
                .unwrap_or_default(),
            FuseBlown => self
                .fail_status_2()
                .map(|status| status.fuse_blown())
                .unwrap_or_default(),
            FetUncontrol => self
                .fail_status_2()
                .map(|status| status.fet_uncontrol())
                .unwrap_or_default(),
            SelfTestClockFail => self
                .fail_status_3()
                .map(|status| status.self_test_clock_fail())
                .unwrap_or_default(),
            SelfTestRomFail => self
                .fail_status_3()
                .map(|status| status.self_test_rom_fail())
                .unwrap_or_default(),
            SelfTestRegisterFail => self
                .fail_status_3()
                .map(|status| status.self_test_register_fail())
                .unwrap_or_default(),
            SelfTestPswRegisterFail => self
                .fail_status_3()
                .map(|status| status.self_test_psw_register_fail())
                .unwrap_or_default(),
            SelfTestStackRegisterFail => self
                .fail_status_3()
                .map(|status| status.self_test_stack_register_fail())
                .unwrap_or_default(),
            SelfTestCsRegisterFail => self
                .fail_status_3()
                .map(|status| status.self_test_cs_register_fail())
                .unwrap_or_default(),
            SelfTestEsRegisterFail => self
                .fail_status_3()
                .map(|status| status.self_test_es_register_fail())
                .unwrap_or_default(),
            SelfTestRamFailDfFail => self
                .fail_status_3()
                .map(|status| status.self_test_ram_fail_df_fail())
                .unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_over_current_discharge_detection_65a() {
        let status = FailStatus1(0x00);
        assert_eq!(status.over_current_discharge_detection_65a(), FailState::Ok);
        let status = FailStatus1(0xff);
        assert_eq!(status.over_current_discharge_detection_65a(), FailState::Ng);
        let status = FailStatus1(0b00000001);
        assert_eq!(status.over_current_discharge_detection_65a(), FailState::Ng);
        let status = FailStatus1(0b11111110);
        assert_eq!(status.over_current_discharge_detection_65a(), FailState::Ok);
    }

    #[test]
    fn test_over_current_discharge_detection_90a() {
        let status = FailStatus1(0x00);
        assert_eq!(status.over_current_discharge_detection_90a(), FailState::Ok);
        let status = FailStatus1(0xff);
        assert_eq!(status.over_current_discharge_detection_90a(), FailState::Ng);
        let status = FailStatus1(0b00000010);
        assert_eq!(status.over_current_discharge_detection_90a(), FailState::Ng);
        let status = FailStatus1(0b11111101);
        assert_eq!(status.over_current_discharge_detection_90a(), FailState::Ok);
    }

    #[test]
    fn test_over_charge_protection() {
        let status = FailStatus1(0x00);
        assert_eq!(status.over_charge_protection(), FailState::Ok);
        let status = FailStatus1(0xff);
        assert_eq!(status.over_charge_protection(), FailState::Ng);
        let status = FailStatus1(0b00000100);
        assert_eq!(status.over_charge_protection(), FailState::Ng);
        let status = FailStatus1(0b11111011);
        assert_eq!(status.over_charge_protection(), FailState::Ok);
    }

    #[test]
    fn test_over_current_charge_detection_45a() {
        let status = FailStatus1(0x00);
        assert_eq!(status.over_current_charge_detection_45a(), FailState::Ok);
        let status = FailStatus1(0xff);
        assert_eq!(status.over_current_charge_detection_45a(), FailState::Ng);
        let status = FailStatus1(0b00001000);
        assert_eq!(status.over_current_charge_detection_45a(), FailState::Ng);
        let status = FailStatus1(0b11110111);
        assert_eq!(status.over_current_charge_detection_45a(), FailState::Ok);
    }

    #[test]
    fn test_over_temperature_discharge_detection() {
        let status = FailStatus1(0x00);
        assert_eq!(status.over_temperature_discharge_detection(), FailState::Ok);
        let status = FailStatus1(0xff);
        assert_eq!(status.over_temperature_discharge_detection(), FailState::Ng);
        let status = FailStatus1(0b00010000);
        assert_eq!(status.over_temperature_discharge_detection(), FailState::Ng);
        let status = FailStatus1(0b11101111);
        assert_eq!(status.over_temperature_discharge_detection(), FailState::Ok);
    }

    #[test]
    fn test_low_voltage_detection() {
        let status = FailStatus1(0x00);
        assert_eq!(status.low_voltage_detection(), FailState::Ok);
        let status = FailStatus1(0xff);
        assert_eq!(status.low_voltage_detection(), FailState::Ng);
        let status = FailStatus1(0b00100000);
        assert_eq!(status.low_voltage_detection(), FailState::Ng);
        let status = FailStatus1(0b11011111);
        assert_eq!(status.low_voltage_detection(), FailState::Ok);
    }

    #[test]
    fn test_fully_charge_detection() {
        let status = FailStatus1(0x00);
        assert_eq!(status.fully_charge_detection(), FailState::Ok);
        let status = FailStatus1(0xff);
        assert_eq!(status.fully_charge_detection(), FailState::Ng);
        let status = FailStatus1(0b01000000);
        assert_eq!(status.fully_charge_detection(), FailState::Ng);
        let status = FailStatus1(0b10111111);
        assert_eq!(status.fully_charge_detection(), FailState::Ok);
    }

    #[test]
    fn test_over_current_discharge_detection_200a() {
        let status = FailStatus1(0x00);
        assert_eq!(
            status.over_current_discharge_detection_200a(),
            FailState::Ok
        );
        let status = FailStatus1(0xff);
        assert_eq!(
            status.over_current_discharge_detection_200a(),
            FailState::Ng
        );
        let status = FailStatus1(0b10000000);
        assert_eq!(
            status.over_current_discharge_detection_200a(),
            FailState::Ng
        );
        let status = FailStatus1(0b01111111);
        assert_eq!(
            status.over_current_discharge_detection_200a(),
            FailState::Ok
        );
    }

    #[test]
    fn test_over_current_discharge_detection_110a() {
        let status = FailStatus2(0x00);
        assert_eq!(
            status.over_current_discharge_detection_110a(),
            FailState::Ok
        );
        let status = FailStatus2(0xff);
        assert_eq!(
            status.over_current_discharge_detection_110a(),
            FailState::Ng
        );
        let status = FailStatus2(0b00000001);
        assert_eq!(
            status.over_current_discharge_detection_110a(),
            FailState::Ng
        );
        let status = FailStatus2(0b11111110);
        assert_eq!(
            status.over_current_discharge_detection_110a(),
            FailState::Ok
        );
    }

    #[test]
    fn test_over_current_charge_detection_65a() {
        let status = FailStatus2(0x00);
        assert_eq!(status.over_current_charge_detection_65a(), FailState::Ok);
        let status = FailStatus2(0xff);
        assert_eq!(status.over_current_charge_detection_65a(), FailState::Ng);
        let status = FailStatus2(0b00000010);
        assert_eq!(status.over_current_charge_detection_65a(), FailState::Ng);
        let status = FailStatus2(0b11111101);
        assert_eq!(status.over_current_charge_detection_65a(), FailState::Ok);
    }

    #[test]
    fn test_over_temperature_charge_detection() {
        let status = FailStatus2(0x00);
        assert_eq!(status.over_temperature_charge_detection(), FailState::Ok);
        let status = FailStatus2(0xff);
        assert_eq!(status.over_temperature_charge_detection(), FailState::Ng);
        let status = FailStatus2(0b00000100);
        assert_eq!(status.over_temperature_charge_detection(), FailState::Ng);
        let status = FailStatus2(0b11111011);
        assert_eq!(status.over_temperature_charge_detection(), FailState::Ok);
    }

    #[test]
    fn test_cell_unbalance_detection() {
        let status = FailStatus2(0x00);
        assert_eq!(status.cell_unbalance_detection(), FailState::Ok);
        let status = FailStatus2(0xff);
        assert_eq!(status.cell_unbalance_detection(), FailState::Ng);
        let status = FailStatus2(0b00001000);
        assert_eq!(status.cell_unbalance_detection(), FailState::Ng);
        let status = FailStatus2(0b11110111);
        assert_eq!(status.cell_unbalance_detection(), FailState::Ok);
    }

    #[test]
    fn test_over_charge() {
        let status = FailStatus2(0x00);
        assert_eq!(status.over_charge(), FailState::Ok);
        let status = FailStatus2(0xff);
        assert_eq!(status.over_charge(), FailState::Ng);
        let status = FailStatus2(0b00010000);
        assert_eq!(status.over_charge(), FailState::Ng);
        let status = FailStatus2(0b11101111);
        assert_eq!(status.over_charge(), FailState::Ok);
    }

    #[test]
    fn test_deep_discharge() {
        let status = FailStatus2(0x00);
        assert_eq!(status.deep_discharge(), FailState::Ok);
        let status = FailStatus2(0xff);
        assert_eq!(status.deep_discharge(), FailState::Ng);
        let status = FailStatus2(0b00100000);
        assert_eq!(status.deep_discharge(), FailState::Ng);
        let status = FailStatus2(0b11011111);
        assert_eq!(status.deep_discharge(), FailState::Ok);
    }

    #[test]
    fn test_fuse_blown() {
        let status = FailStatus2(0x00);
        assert_eq!(status.fuse_blown(), FailState::Ok);
        let status = FailStatus2(0xff);
        assert_eq!(status.fuse_blown(), FailState::Ng);
        let status = FailStatus2(0b01000000);
        assert_eq!(status.fuse_blown(), FailState::Ng);
        let status = FailStatus2(0b10111111);
        assert_eq!(status.fuse_blown(), FailState::Ok);
    }

    #[test]
    fn test_fet_uncontrol() {
        let status = FailStatus2(0x00);
        assert_eq!(status.fet_uncontrol(), FailState::Ok);
        let status = FailStatus2(0xff);
        assert_eq!(status.fet_uncontrol(), FailState::Ng);
        let status = FailStatus2(0b10000000);
        assert_eq!(status.fet_uncontrol(), FailState::Ng);
        let status = FailStatus2(0b01111111);
        assert_eq!(status.fet_uncontrol(), FailState::Ok);
    }

    #[test]
    fn test_self_test_clock_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_clock_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_clock_fail(), FailState::Ng);
        let status = FailStatus3(0b00000001);
        assert_eq!(status.self_test_clock_fail(), FailState::Ng);
        let status = FailStatus3(0b11111110);
        assert_eq!(status.self_test_clock_fail(), FailState::Ok);
    }

    #[test]
    fn test_self_test_rom_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_rom_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_rom_fail(), FailState::Ng);
        let status = FailStatus3(0b00000010);
        assert_eq!(status.self_test_rom_fail(), FailState::Ng);
        let status = FailStatus3(0b11111101);
        assert_eq!(status.self_test_rom_fail(), FailState::Ok);
    }

    #[test]
    fn test_self_test_register_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_register_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_register_fail(), FailState::Ng);
        let status = FailStatus3(0b00000100);
        assert_eq!(status.self_test_register_fail(), FailState::Ng);
        let status = FailStatus3(0b11111011);
        assert_eq!(status.self_test_register_fail(), FailState::Ok);
    }

    #[test]
    fn test_self_test_psw_register_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_psw_register_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_psw_register_fail(), FailState::Ng);
        let status = FailStatus3(0b00001000);
        assert_eq!(status.self_test_psw_register_fail(), FailState::Ng);
        let status = FailStatus3(0b11110111);
        assert_eq!(status.self_test_psw_register_fail(), FailState::Ok);
    }

    #[test]
    fn test_self_test_stack_register_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_stack_register_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_stack_register_fail(), FailState::Ng);
        let status = FailStatus3(0b00010000);
        assert_eq!(status.self_test_stack_register_fail(), FailState::Ng);
        let status = FailStatus3(0b11101111);
        assert_eq!(status.self_test_stack_register_fail(), FailState::Ok);
    }

    #[test]
    fn test_self_test_cs_register_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_cs_register_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_cs_register_fail(), FailState::Ng);
        let status = FailStatus3(0b00100000);
        assert_eq!(status.self_test_cs_register_fail(), FailState::Ng);
        let status = FailStatus3(0b11011111);
        assert_eq!(status.self_test_cs_register_fail(), FailState::Ok);
    }

    #[test]
    fn test_self_test_es_register_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_es_register_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_es_register_fail(), FailState::Ng);
        let status = FailStatus3(0b01000000);
        assert_eq!(status.self_test_es_register_fail(), FailState::Ng);
        let status = FailStatus3(0b10111111);
        assert_eq!(status.self_test_es_register_fail(), FailState::Ok);
    }

    #[test]
    fn test_self_test_ram_fail_df_fail() {
        let status = FailStatus3(0x00);
        assert_eq!(status.self_test_ram_fail_df_fail(), FailState::Ok);
        let status = FailStatus3(0xff);
        assert_eq!(status.self_test_ram_fail_df_fail(), FailState::Ng);
        let status = FailStatus3(0b10000000);
        assert_eq!(status.self_test_ram_fail_df_fail(), FailState::Ng);
        let status = FailStatus3(0b01111111);
        assert_eq!(status.self_test_ram_fail_df_fail(), FailState::Ok);
    }

    use crate::error::{Error, Result};

    struct MockBatteryState(Option<u8>, Option<u8>, Option<u8>);

    impl BatteryState for MockBatteryState {
        fn cell_voltages(&self) -> Result<Vec<u32>> {
            Ok(Default::default())
        }

        fn current(&self) -> Result<i32> {
            Ok(Default::default())
        }

        fn temperature(&self) -> Result<f64> {
            Ok(Default::default())
        }

        fn remaining_capacity(&self) -> Result<u32> {
            Ok(Default::default())
        }

        fn full_charge_capacity(&self) -> Result<u32> {
            Ok(Default::default())
        }

        fn design_capacity(&self) -> Result<u32> {
            Ok(Default::default())
        }

        fn absolute_state_of_charge(&self) -> Result<u32> {
            Ok(Default::default())
        }

        fn relative_state_of_charge(&self) -> Result<u32> {
            Ok(Default::default())
        }

        fn state_of_health(&self) -> Result<u32> {
            Ok(Default::default())
        }

        fn bm_voltage(&self) -> Result<u32> {
            Ok(Default::default())
        }

        fn fail_status_1(&self) -> Result<FailStatus1> {
            self.0
                .map(FailStatus1)
                .ok_or_else(|| Error::DataBytesShortage("test".to_owned()))
        }

        fn fail_status_2(&self) -> Result<FailStatus2> {
            self.1
                .map(FailStatus2)
                .ok_or_else(|| Error::DataBytesShortage("test".to_owned()))
        }

        fn fail_status_3(&self) -> Result<FailStatus3> {
            self.2
                .map(FailStatus3)
                .ok_or_else(|| Error::DataBytesShortage("test".to_owned()))
        }
    }

    #[test]
    fn test_fail_status() {
        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            assert_eq!(fail_status, FailState::Ok);
        }

        let battery_state = MockBatteryState(Some(0xff), Some(0xff), Some(0xff));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            assert_eq!(fail_status, FailState::Ng);
        }
    }

    #[test]
    fn test_fail_status_individually() {
        use FailStatusItem::*;

        let battery_state = MockBatteryState(Some(0b00000001), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentDischargeDetection65A => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0b00000010), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentDischargeDetection90A => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0b00000100), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverChargeProtection => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0b00001000), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentChargeDetection45A => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0b00010000), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverTemperatureDischargeDetection => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0b00100000), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                LowVoltageDetection => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0b01000000), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                FullyChargeDetection => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0b10000000), Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentDischargeDetection200A => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b00000001), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentDischargeDetection110A => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b00000010), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentChargeDetection65A => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b00000100), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverTemperatureChargeDetection => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b00001000), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                CellUnbalanceDetection => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b00010000), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCharge => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b00100000), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                DeepDischarge => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b01000000), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                FuseBlown => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0b10000000), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                FetUncontrol => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b00000001));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestClockFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b00000010));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestRomFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b00000100));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestRegisterFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b00001000));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestPswRegisterFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b00010000));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestStackRegisterFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b00100000));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestCsRegisterFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b01000000));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestEsRegisterFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), Some(0b10000000));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestRamFailDfFail => assert_eq!(fail_status, FailState::Ng),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }
    }

    #[test]
    fn test_fail_status_unknown() {
        use FailStatusItem::*;

        let battery_state = MockBatteryState(None, Some(0x00), Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentDischargeDetection65A => assert_eq!(fail_status, FailState::Unknown),
                OverCurrentDischargeDetection90A => assert_eq!(fail_status, FailState::Unknown),
                OverChargeProtection => assert_eq!(fail_status, FailState::Unknown),
                OverCurrentChargeDetection45A => assert_eq!(fail_status, FailState::Unknown),
                OverTemperatureDischargeDetection => assert_eq!(fail_status, FailState::Unknown),
                LowVoltageDetection => assert_eq!(fail_status, FailState::Unknown),
                FullyChargeDetection => assert_eq!(fail_status, FailState::Unknown),
                OverCurrentDischargeDetection200A => assert_eq!(fail_status, FailState::Unknown),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), None, Some(0x00));
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                OverCurrentDischargeDetection110A => assert_eq!(fail_status, FailState::Unknown),
                OverCurrentChargeDetection65A => assert_eq!(fail_status, FailState::Unknown),
                OverTemperatureChargeDetection => assert_eq!(fail_status, FailState::Unknown),
                CellUnbalanceDetection => assert_eq!(fail_status, FailState::Unknown),
                OverCharge => assert_eq!(fail_status, FailState::Unknown),
                DeepDischarge => assert_eq!(fail_status, FailState::Unknown),
                FuseBlown => assert_eq!(fail_status, FailState::Unknown),
                FetUncontrol => assert_eq!(fail_status, FailState::Unknown),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }

        let battery_state = MockBatteryState(Some(0x00), Some(0x00), None);
        for item in FailStatusItem::iter() {
            let fail_status = battery_state.fail_status(item);
            match item {
                SelfTestClockFail => assert_eq!(fail_status, FailState::Unknown),
                SelfTestRomFail => assert_eq!(fail_status, FailState::Unknown),
                SelfTestRegisterFail => assert_eq!(fail_status, FailState::Unknown),
                SelfTestPswRegisterFail => assert_eq!(fail_status, FailState::Unknown),
                SelfTestStackRegisterFail => assert_eq!(fail_status, FailState::Unknown),
                SelfTestCsRegisterFail => assert_eq!(fail_status, FailState::Unknown),
                SelfTestEsRegisterFail => assert_eq!(fail_status, FailState::Unknown),
                SelfTestRamFailDfFail => assert_eq!(fail_status, FailState::Unknown),
                _ => assert_eq!(fail_status, FailState::Ok),
            }
        }
    }
}
