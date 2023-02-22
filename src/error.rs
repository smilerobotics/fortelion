use std::path::PathBuf;

use thiserror::Error;

use crate::uart;

#[derive(Debug, Error)]
pub enum Error {
    #[error("fortelion: Failed to open: path({:?}) Error({:?})", path, source)]
    UartFailedToOpen {
        #[source]
        source: serialport::Error,
        path: PathBuf,
    },
    #[error("fortelion: Failed to send: Error({:?})", .0)]
    UartFailedToSend(std::io::Error),
    #[error("fortelion: Failed to receive: Error({:?})", .0)]
    UartFailedToReceive(std::io::Error),
    #[error("fortelion: Invalid data frame {:?}", .0)]
    InvalidUartDataFrame(String),
    #[error(
        "fortelion: Uart Data frame has no appropriate data: response command is `{:?}`, must be any of {:?}",
        response_command,
        must_be_any_of
    )]
    NoAppropriateData {
        response_command: uart::Command,
        must_be_any_of: Vec<uart::Command>,
    },
    #[error("fortelion: Data bytes shortage {:?}", .0)]
    DataBytesShortage(String),
}

pub type Result<T> = ::std::result::Result<T, Error>;
