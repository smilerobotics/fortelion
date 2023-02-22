use super::{command::Command, command_frame::LEADER_BM_ID, utils::checksum};
use crate::error::{Error, Result};

const START_CODE_INDEX: usize = 0;
const BM_ID_INDEX: usize = 1;
const RESPONSE_COMMAND_INDEX: usize = 2;
const NUMBER_OF_DATA_INDEX: usize = 3;
const DATA_OFFSET: usize = 4;

const NUMBER_OF_BYTES_EXCEPT_FOR_DATA: usize = 6 /* Start Code, BM ID, Response Command, Number of data, Checksum, Reserved */;

const DATA_FRAME_START_CODE: u8 = 0x02;

#[derive(Debug)]
pub struct DataFrame {
    response_command: Command,
    buf: Vec<u8>,
}

impl DataFrame {
    pub fn new(response_command: Command) -> Self {
        let frame_length = response_command.number_of_data() + NUMBER_OF_BYTES_EXCEPT_FOR_DATA;
        let buf = vec![0; frame_length];

        Self {
            response_command,
            buf,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.buf[DATA_OFFSET..DATA_OFFSET + self.response_command.number_of_data()]
    }

    pub fn is_valid(&self) -> Result<()> {
        if self.buf[START_CODE_INDEX] != DATA_FRAME_START_CODE {
            Err(Error::InvalidUartDataFrame(format!(
                "Invalid start code (must be: {DATA_FRAME_START_CODE}, received {})",
                self.buf[START_CODE_INDEX]
            )))
        } else if self.buf[BM_ID_INDEX] != LEADER_BM_ID {
            Err(Error::InvalidUartDataFrame(format!(
                "Invalid BM ID (must be: {LEADER_BM_ID}, received {})",
                self.buf[BM_ID_INDEX]
            )))
        } else if self.buf[RESPONSE_COMMAND_INDEX] != self.response_command as u8 {
            Err(Error::InvalidUartDataFrame(format!(
                "Response command mismatch (must be: {}, received {})",
                self.response_command as u8, self.buf[RESPONSE_COMMAND_INDEX]
            )))
        } else if self.buf[NUMBER_OF_DATA_INDEX] != self.response_command.number_of_data() as u8 {
            Err(Error::InvalidUartDataFrame(format!(
                "Invalid number of data (must be: {}, received {})",
                self.response_command.number_of_data(),
                self.buf[NUMBER_OF_DATA_INDEX]
            )))
        } else {
            let received_checksum = self.buf[DATA_OFFSET + self.response_command.number_of_data()];
            let calculated_checksum =
                checksum(&self.buf[..DATA_OFFSET + self.response_command.number_of_data()]);
            if received_checksum != calculated_checksum {
                Err(Error::InvalidUartDataFrame("Invalid checksum".to_owned()))
            } else {
                Ok(())
            }
        }
    }

    pub fn response_command(&self) -> Command {
        self.response_command
    }
}

impl AsMut<[u8]> for DataFrame {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn test_is_valid() {
        const RESERVED: u8 = 0x00;

        // Valid data frame
        let header = &[
            DATA_FRAME_START_CODE,
            LEADER_BM_ID,
            Command::Current as u8,
            Command::Current.number_of_data() as u8,
        ];
        let current = 1234u16;
        let data = current.to_be_bytes();
        let mut buf = header.to_vec();
        buf.extend_from_slice(&data);
        buf.push(0xd4); // checksum
        buf.push(RESERVED);

        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&buf).unwrap();
        assert!(data_frame.is_valid().is_ok());

        // Invalid checksum
        let mut buf = header.to_vec();
        buf.extend_from_slice(&data);
        buf.push(0xff); // checksum
        buf.push(RESERVED);

        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&buf).unwrap();
        let result = data_frame.is_valid();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid checksum"));

        // Invalid start code
        let header = &[
            0x01,
            LEADER_BM_ID,
            Command::Current as u8,
            Command::Current.number_of_data() as u8,
        ];
        let mut buf = header.to_vec();
        buf.extend_from_slice(&data);
        buf.push(checksum(&buf));
        buf.push(RESERVED);

        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&buf).unwrap();
        let result = data_frame.is_valid();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid start code"));

        // Invalid BM ID
        let header = &[
            DATA_FRAME_START_CODE,
            0x00,
            Command::Current as u8,
            Command::Current.number_of_data() as u8,
        ];
        let mut buf = header.to_vec();
        buf.extend_from_slice(&data);
        buf.push(checksum(&buf));
        buf.push(RESERVED);

        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&buf).unwrap();
        let result = data_frame.is_valid();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid BM ID"));

        // Command mismatch
        let header = &[
            DATA_FRAME_START_CODE,
            LEADER_BM_ID,
            Command::Temperature as u8,
            Command::Current.number_of_data() as u8,
        ];
        let mut buf = header.to_vec();
        buf.extend_from_slice(&data);
        buf.push(checksum(&buf));
        buf.push(RESERVED);

        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&buf).unwrap();
        let result = data_frame.is_valid();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Response command mismatch"));

        // Invalid number of data
        let header = &[
            DATA_FRAME_START_CODE,
            LEADER_BM_ID,
            Command::Current as u8,
            3,
        ];
        let mut buf = header.to_vec();
        buf.extend_from_slice(&data);
        buf.push(checksum(&buf));
        buf.push(RESERVED);

        let mut data_frame = DataFrame::new(Command::Current);
        data_frame.as_mut().write_all(&buf).unwrap();
        let result = data_frame.is_valid();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid number of data"));
    }
}
