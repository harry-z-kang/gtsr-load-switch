#![feature(associated_type_bounds)]
mod gtsr_gpio;

use std::marker::PhantomData;

pub const LS_OVERCURRENT_DISABLE: f64 = 99999.9;
pub const LS_MIN_EXPECT_CURRENT_DISABLE: f64 = -99999.9;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadSwitchChannel {
    LoadSwitchCh1 = 0,
    LoadSwitchCh2 = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadSwitchChannelStatus {
    LsStatusNominalOn = 1,
    LsStatusNominalOff = 0,
    LsStatusWarningLowCurrent = -1,
    LsStatusFaultOpenCircuit = -2,
    LsStatusFaultOverCurrent = -3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SensorEnablingStatus {
    Chan1Sensing = 1, // sel[0] and sel[1] are not connected. Only channel 1 current sensing is available.
    Chan12Sensing = 2, // sel[0] is not connected: Ch1,2 current sense enabled, temperature sensing not enabled
    TempSensing = 3,   // sel[0] is connected, temperature sensing is enabled
}

#[allow(deprecated)]
pub struct LoadSwitch<
    'a,
    L: gtsr_gpio::Pin,
    S1: gtsr_gpio::Pin,
    S2: gtsr_gpio::Pin,
    EN1: gtsr_gpio::Pin,
    EN2: gtsr_gpio::Pin,
    DIA: gtsr_gpio::Pin,
> {
    latch: &'a gtsr_gpio::Signal<L>,
    sel1: &'a gtsr_gpio::Signal<S1>,
    sel2: &'a gtsr_gpio::Signal<S2>,
    en1: &'a gtsr_gpio::Signal<EN1>,
    en2: &'a gtsr_gpio::Signal<EN2>,
    dia_en: &'a gtsr_gpio::Signal<DIA>,
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

    _latch_pin: PhantomData<L>,
    _sel1_pin: PhantomData<S1>,
    _sel2_pin: PhantomData<S2>,
    _en1_pin: PhantomData<EN1>,
    _en2_pin: PhantomData<EN2>,
    _dia_en_pin: PhantomData<DIA>,
}

impl<
        L: gtsr_gpio::Pin,
        S1: gtsr_gpio::Pin,
        S2: gtsr_gpio::Pin,
        EN1: gtsr_gpio::Pin,
        EN2: gtsr_gpio::Pin,
        DIA: gtsr_gpio::Pin,
    > LoadSwitch<'static, L, S1, S2, EN1, EN2, DIA>
{
}
