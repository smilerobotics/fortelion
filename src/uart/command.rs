#[derive(Clone, Copy, Debug)]
pub enum Command {
    FailStatus1 = 0x01,
    CellVoltage = 0x02,
    Current = 0x03,
    Temperature = 0x04,
    RemainingCapacity = 0x05,
    BmInformation = 0x10,
    FullChargeCapacity = 0x11,
    FailStatus2 = 0x13,
    StateOfHealth = 0x14,
    SummaryData = 0x20,
    VersionInformation = 0x50,
    DesignCapacity = 0x55,
}

impl Command {
    pub fn number_of_data_in_command(&self) -> usize {
        0
    }

    pub fn number_of_data(&self) -> usize {
        match *self {
            Self::FailStatus1 => 1,
            Self::CellVoltage => 16,
            Self::Current => 2,
            Self::Temperature => 2,
            Self::RemainingCapacity => 2,
            Self::BmInformation => 29,
            Self::FullChargeCapacity => 2,
            Self::FailStatus2 => 1,
            Self::StateOfHealth => 1,
            Self::SummaryData => 50,
            Self::VersionInformation => 3,
            Self::DesignCapacity => 2,
        }
    }
}
