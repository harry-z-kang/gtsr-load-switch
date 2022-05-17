use tm4c123x_hal as hal;

use self::hal::gpio;

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

pub struct LoadSwitch<PXx> {
    latch: PXx<gpio::Output<gpio::PushPull>>,
}
