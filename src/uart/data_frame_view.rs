use super::{Command, DataFrame};
use crate::{
    battery_state::BatteryState,
    error::{Error, Result},
    fail_status::*,
    utils::*,
};

#[derive(Debug)]
pub struct DataFrameView<'a> {
    data_frame: &'a DataFrame,
}

impl<'a> DataFrameView<'a> {
    pub fn try_new(data_frame: &'a DataFrame) -> Result<Self> {
        data_frame.is_valid()?;
        Ok(Self { data_frame })
    }
}

const NUMBER_OF_CELLS: usize = 8; // on the assumption that the battery module is an `All-in-one type`

const CELL_VOLTAGE_INDEX_IN_BM_INFORMATION: usize = 1;
const CURRENT_INDEX_IN_BM_INFORMATION: usize = 17;
const TEMPERATURE_INDEX_IN_BM_INFORMATION: usize = 19;
const REMAINING_CAPACITY_INDEX_IN_BM_INFORMATION: usize = 21;
const FULL_CHARGE_CAPACITY_INDEX_IN_BM_INFORMATION: usize = 23;
const DESIGN_CAPACITY_INDEX_IN_BM_INFORMATION: usize = 25;
const STATE_OF_HEALTH_INDEX_IN_BM_INFORMATION: usize = 28;
const FAIL_STATUS_1_INDEX_IN_BM_INFORMATION: usize = 0;
const FAIL_STATUS_2_INDEX_IN_BM_INFORMATION: usize = 27;

const CURRENT_INDEX_IN_SUMMARY_DATA: usize = 7;
const ABSOLUTE_STATE_OF_CHARGE_INDEX_IN_SUMMARY_DATA: usize = 2;
const RELATIVE_STATE_OF_CHARGE_INDEX_IN_SUMMARY_DATA: usize = 3;
const STATE_OF_HEALTH_INDEX_IN_SUMMARY_DATA: usize = 4;
const BM_VOLTAGE_MAX_INDEX_IN_SUMMARY_DATA: usize = 11;
const DESIGN_CAPACITY_INDEX_IN_SUMMARY_DATA: usize = 17;
const FULL_CHARGE_CAPACITY_INDEX_IN_SUMMARY_DATA: usize = 19;
const REMAINING_CAPACITY_INDEX_IN_SUMMARY_DATA: usize = 21;
const MAX_TEMPERATURE_INDEX_IN_SUMMARY_DATA: usize = 32;
#[allow(dead_code)]
const MIN_TEMPERATURE_INDEX_IN_SUMMARY_DATA: usize = 35;
const FAIL_STATUS_1_INDEX_IN_SUMMARY_DATA: usize = 0;
const FAIL_STATUS_2_INDEX_IN_SUMMARY_DATA: usize = 13;
const FAIL_STATUS_3_INDEX_IN_SUMMARY_DATA: usize = 14;

impl<'a> BatteryState for DataFrameView<'a> {
    /// Returns voltages of each cells in the battery module
    /// Unit: mV
    fn cell_voltages(&self) -> Result<Vec<u32>> {
        let data = self.data_frame.data();
        let data = match self.data_frame.response_command() {
            Command::CellVoltage => data,
            Command::BmInformation => cut_slice(
                data,
                CELL_VOLTAGE_INDEX_IN_BM_INFORMATION,
                NUMBER_OF_CELLS * 2,
            )?,
            _ => {
                return Err(Error::NoAppropriateData {
                    response_command: self.data_frame.response_command(),
                    must_be_any_of: vec![Command::CellVoltage, Command::BmInformation],
                })
            }
        };
        Ok(data
            .chunks(2)
            .map(|bytes| bytes_to_u16(bytes).unwrap() as u32)
            .collect())
    }

    /// Returns current
    /// Unit: mA
    /// Positive value means the battery module is charging.
    /// Negative value means the battery module is discharging.
    fn current(&self) -> Result<i32> {
        let data = self.data_frame.data();
        match self.data_frame.response_command() {
            Command::Current => Ok(bytes_to_i16(data)? as i32),
            Command::BmInformation => {
                Ok(bytes_to_i16(cut_slice(data, CURRENT_INDEX_IN_BM_INFORMATION, 2)?)? as i32 * 10)
            }
            Command::SummaryData => {
                Ok(bytes_to_i16(cut_slice(data, CURRENT_INDEX_IN_SUMMARY_DATA, 2)?)? as i32 * 10)
            }
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![
                    Command::Current,
                    Command::BmInformation,
                    Command::SummaryData,
                ],
            }),
        }
    }

    /// Returns temperature of the battery module
    /// Unit: degC
    fn temperature(&self) -> Result<f64> {
        let data = self.data_frame.data();
        match self.data_frame.response_command() {
            Command::Temperature => Ok(bytes_to_i16(data)? as f64),
            Command::BmInformation => {
                Ok(
                    bytes_to_i16(cut_slice(data, TEMPERATURE_INDEX_IN_BM_INFORMATION, 2)?)? as f64
                        * 0.1,
                )
            }
            Command::SummaryData => Ok(bytes_to_i16(cut_slice(
                data,
                MAX_TEMPERATURE_INDEX_IN_SUMMARY_DATA,
                2,
            )?)? as f64
                * 0.1),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![
                    Command::Temperature,
                    Command::BmInformation,
                    Command::SummaryData,
                ],
            }),
        }
    }

    /// Returns remaining capacity
    /// Unit: mAh
    fn remaining_capacity(&self) -> Result<u32> {
        let data = self.data_frame.data();
        match self.data_frame.response_command() {
            Command::RemainingCapacity => Ok(bytes_to_u16(data)? as u32),
            Command::BmInformation => Ok(bytes_to_u16(cut_slice(
                data,
                REMAINING_CAPACITY_INDEX_IN_BM_INFORMATION,
                2,
            )?)? as u32),
            Command::SummaryData => Ok(bytes_to_u16(cut_slice(
                data,
                REMAINING_CAPACITY_INDEX_IN_SUMMARY_DATA,
                2,
            )?)? as u32
                * 10),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![
                    Command::RemainingCapacity,
                    Command::BmInformation,
                    Command::SummaryData,
                ],
            }),
        }
    }

    /// Returns design capacity of the battery module
    /// Unit: mAh
    fn design_capacity(&self) -> Result<u32> {
        let data = self.data_frame.data();
        match self.data_frame.response_command() {
            Command::DesignCapacity => Ok(bytes_to_u16(data)? as u32),
            Command::BmInformation => Ok(bytes_to_u16(cut_slice(
                data,
                DESIGN_CAPACITY_INDEX_IN_BM_INFORMATION,
                2,
            )?)? as u32),
            Command::SummaryData => Ok(bytes_to_u16(cut_slice(
                data,
                DESIGN_CAPACITY_INDEX_IN_SUMMARY_DATA,
                2,
            )?)? as u32
                * 10),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![
                    Command::DesignCapacity,
                    Command::BmInformation,
                    Command::SummaryData,
                ],
            }),
        }
    }

    /// Returns currently fully charged capacity
    /// Unit: mAh
    fn full_charge_capacity(&self) -> Result<u32> {
        let data = self.data_frame.data();
        match self.data_frame.response_command() {
            Command::FullChargeCapacity => Ok(bytes_to_u16(data)? as u32),
            Command::BmInformation => Ok(bytes_to_u16(cut_slice(
                data,
                FULL_CHARGE_CAPACITY_INDEX_IN_BM_INFORMATION,
                2,
            )?)? as u32),
            Command::SummaryData => Ok(bytes_to_u16(cut_slice(
                data,
                FULL_CHARGE_CAPACITY_INDEX_IN_SUMMARY_DATA,
                2,
            )?)? as u32
                * 10),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![
                    Command::FullChargeCapacity,
                    Command::BmInformation,
                    Command::SummaryData,
                ],
            }),
        }
    }

    /// Returns absolute state of charge
    /// (`remaining capacity` / `design capacity`)
    /// Unit: %
    fn absolute_state_of_charge(&self) -> Result<u32> {
        match self.data_frame.response_command() {
            Command::BmInformation => {
                let remaining = self.remaining_capacity()? as f64;
                let designed = self.design_capacity()? as f64;
                Ok((100.0 * remaining / designed).round() as u32)
            }
            Command::SummaryData => Ok((*self
                .data_frame
                .data()
                .get(ABSOLUTE_STATE_OF_CHARGE_INDEX_IN_SUMMARY_DATA)
                .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))?)
                as u32),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![Command::BmInformation, Command::SummaryData],
            }),
        }
    }

    /// Returns relative state of charge
    /// (`remaining capacity` / `full charge capacity`)
    /// Unit: %
    fn relative_state_of_charge(&self) -> Result<u32> {
        match self.data_frame.response_command() {
            Command::BmInformation => {
                let remaining = self.remaining_capacity()?;
                let full_charge = self.full_charge_capacity()?;
                Ok(100 * remaining / full_charge)
            }
            Command::SummaryData => Ok((*self
                .data_frame
                .data()
                .get(RELATIVE_STATE_OF_CHARGE_INDEX_IN_SUMMARY_DATA)
                .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))?)
                as u32),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![Command::BmInformation, Command::SummaryData],
            }),
        }
    }

    /// Returns state of health
    /// (`full charge capacity` / `design capacity`)
    /// Unit: %
    fn state_of_health(&self) -> Result<u32> {
        let data = self.data_frame.data();
        match self.data_frame.response_command() {
            Command::StateOfHealth => Ok((*data
                .first()
                .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))?)
                as u32),
            Command::BmInformation => Ok((*data
                .get(STATE_OF_HEALTH_INDEX_IN_BM_INFORMATION)
                .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))?)
                as u32),
            Command::SummaryData => Ok((*data
                .get(STATE_OF_HEALTH_INDEX_IN_SUMMARY_DATA)
                .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))?)
                as u32),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![
                    Command::StateOfHealth,
                    Command::BmInformation,
                    Command::SummaryData,
                ],
            }),
        }
    }

    /// Returns voltage of the whole battery module
    /// Unit: mV
    fn bm_voltage(&self) -> Result<u32> {
        match self.data_frame.response_command() {
            Command::BmInformation => Ok(self.cell_voltages()?.into_iter().sum()),
            Command::SummaryData => Ok(bytes_to_u16(cut_slice(
                self.data_frame.data(),
                BM_VOLTAGE_MAX_INDEX_IN_SUMMARY_DATA,
                2,
            )?)? as u32),
            _ => Err(Error::NoAppropriateData {
                response_command: self.data_frame.response_command(),
                must_be_any_of: vec![Command::BmInformation, Command::SummaryData],
            }),
        }
    }

    fn fail_status_1(&self) -> Result<FailStatus1> {
        self.data_frame
            .data()
            .get(match self.data_frame.response_command() {
                Command::FailStatus1 => 0,
                Command::BmInformation => FAIL_STATUS_1_INDEX_IN_BM_INFORMATION,
                Command::SummaryData => FAIL_STATUS_1_INDEX_IN_SUMMARY_DATA,
                _ => {
                    return Err(Error::NoAppropriateData {
                        response_command: self.data_frame.response_command(),
                        must_be_any_of: vec![
                            Command::Current,
                            Command::BmInformation,
                            Command::SummaryData,
                        ],
                    })
                }
            })
            .map(|status| FailStatus1(*status))
            .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))
    }

    fn fail_status_2(&self) -> Result<FailStatus2> {
        self.data_frame
            .data()
            .get(match self.data_frame.response_command() {
                Command::FailStatus2 => 0,
                Command::BmInformation => FAIL_STATUS_2_INDEX_IN_BM_INFORMATION,
                Command::SummaryData => FAIL_STATUS_2_INDEX_IN_SUMMARY_DATA,
                _ => {
                    return Err(Error::NoAppropriateData {
                        response_command: self.data_frame.response_command(),
                        must_be_any_of: vec![
                            Command::Current,
                            Command::BmInformation,
                            Command::SummaryData,
                        ],
                    })
                }
            })
            .map(|status| FailStatus2(*status))
            .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))
    }

    fn fail_status_3(&self) -> Result<FailStatus3> {
        self.data_frame
            .data()
            .get(match self.data_frame.response_command() {
                Command::SummaryData => FAIL_STATUS_3_INDEX_IN_SUMMARY_DATA,
                _ => {
                    return Err(Error::NoAppropriateData {
                        response_command: self.data_frame.response_command(),
                        must_be_any_of: vec![Command::SummaryData],
                    })
                }
            })
            .map(|status| FailStatus3(*status))
            .ok_or_else(|| Error::InvalidUartDataFrame("Not enough data".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn test_try_new() {
        // Valid
        let valid_bytes = &[0x02, 0x01, 0x03, 0x02, 0x04, 0xd2, 0xd4, 0x00];
        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&valid_bytes[..]).unwrap();
        let view = DataFrameView::try_new(&data_frame);
        assert!(view.is_ok());

        // Invalid
        let invalid_bytes = &[0x02, 0x01, 0x03, 0x02, 0x04, 0xd2, 0xff, 0x00];
        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&invalid_bytes[..]).unwrap();
        let view = DataFrameView::try_new(&data_frame);
        assert!(view.is_err());
    }
}
