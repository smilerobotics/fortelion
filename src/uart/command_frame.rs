use super::{command::Command, utils::checksum};

const COMMAND_FRAME_START_CODE: u8 = 0x05;
pub(crate) const LEADER_BM_ID: u8 = 0x01;

#[derive(Debug)]
pub struct CommandFrame {
    request_command: Command,
    buf: Vec<u8>,
}

impl CommandFrame {
    pub fn new(request_command: Command) -> Self {
        let number_of_data = request_command.number_of_data_in_command();
        assert_eq!(number_of_data, 0);

        let mut buf = vec![
            COMMAND_FRAME_START_CODE,
            LEADER_BM_ID,
            request_command as u8,
            number_of_data as u8,
        ];
        buf.push(checksum(&buf));

        Self {
            request_command,
            buf,
        }
    }

    pub fn request_command(&self) -> Command {
        self.request_command
    }
}

impl AsRef<[u8]> for CommandFrame {
    fn as_ref(&self) -> &[u8] {
        &self.buf
    }
}
