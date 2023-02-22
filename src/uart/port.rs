use std::{path::Path, time::Duration};

use serialport::{DataBits, Parity, SerialPort, StopBits};

use crate::error::{Error, Result};

const FORTELION_UART_BAUDRATE: u32 = 38400;

pub struct Port {
    inner: Box<dyn SerialPort>,
}

impl Port {
    pub fn try_new(path: impl AsRef<Path>, timeout: Duration) -> Result<Self> {
        let inner = serialport::new(path.as_ref().to_string_lossy(), FORTELION_UART_BAUDRATE)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .parity(Parity::Even)
            .timeout(timeout)
            .open()
            .map_err(|source| Error::UartFailedToOpen {
                source,
                path: path.as_ref().into(),
            })?;

        Ok(Self { inner })
    }

    pub fn send(&mut self, command_frame: &impl AsRef<[u8]>) -> Result<()> {
        self.inner
            .write_all(command_frame.as_ref())
            .map_err(Error::UartFailedToSend)?;
        Ok(())
    }

    pub fn receive(&mut self, buf: &mut impl AsMut<[u8]>) -> Result<()> {
        self.inner
            .read_exact(buf.as_mut())
            .map_err(Error::UartFailedToReceive)?;
        Ok(())
    }
}
