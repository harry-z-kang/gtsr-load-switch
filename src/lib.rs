use tm4c123x_hal as hal;

use self::hal::prelude;

pub const LS_OVERCURRENT_DISABLE: f64 = 99999.9;
pub const LS_MIN_EXPECT_CURRENT_DISABLE: f64 = -99999.9;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum LoadSwitchChannelStatus {
    LsStatusNominalOn = 1,
    LsStatusNominalOff = 0,
    LsStatusWarningLowCurrent = -1,
    LsStatusFaultOpenCircuit = -2,
    LsStatusFaultOverCurrent = -3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SensorEnablingStatus {
    Chan1Sensing = 1, // sel[0] and sel[1] are not connected. Only channel 1 current sensing is available.
    Chan12Sensing = 2, // sel[0] is not connected: Ch1,2 current sense enabled, temperature sensing not enabled
    TempSensing = 3,   // sel[0] is connected, temperature sensing is enabled
}

#[allow(deprecated)]
pub struct LoadSwitch<'a> {
    latch: &'a dyn prelude::_embedded_hal_digital_OutputPin,
    sel: [&'a dyn prelude::_embedded_hal_digital_OutputPin; 2],
    en: [&'a dyn prelude::_embedded_hal_digital_OutputPin; 2],
    dia_en: &'a dyn prelude::_embedded_hal_digital_OutputPin,
    channel_state: SensorEnablingStatus,

    overcurrent_threshold: [f64; 2],
    min_expected_current: [f64; 2],
    current: [f64; 2],
    status: [LoadSwitchChannelStatus; 2],
    latching: bool,
    overcurrent_retries: [i8; 2],
    temperature: f64,
    current_scale_factor: f64,
    temperature_scale_factor: f64,
}
