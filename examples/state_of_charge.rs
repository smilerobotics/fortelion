use std::time::Duration;

use fortelion::{uart::*, BatteryState};

const DEVICE_PATH: &str = "/dev/ttyUSB0";
const TIMEOUT: Duration = Duration::from_millis(100);

fn main() {
    let mut port = Port::try_new(DEVICE_PATH, TIMEOUT).expect("failed to open device");

    let command = Command::SummaryData;
    let command_frame = CommandFrame::new(command);
    port.send(&command_frame).expect("failed to send");

    let mut data_frame = DataFrame::new(command);
    port.receive(&mut data_frame).expect("failed to receive");
    assert!(data_frame.is_valid().is_ok());

    let info = DataFrameView::try_new(&data_frame).unwrap();
    println!(
        "Absolute state of charge: {}%",
        info.absolute_state_of_charge().unwrap()
    );
    println!(
        "Relative state of charge: {}%",
        info.relative_state_of_charge().unwrap()
    );
}
