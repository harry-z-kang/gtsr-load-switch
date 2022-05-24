mod gtsr_gpio;

use std::marker::PhantomData;

use tm4c123x_hal as hal;

use self::hal::prelude;

pub const LS_OVERCURRENT_DISABLE: f64 = 99999.9;
pub const LS_MIN_EXPECT_CURRENT_DISABLE: f64 = -99999.9;
pub const RATIO_DISNST_DT: f64 = 0.011;
pub const TEMP_CONVERSION_OFFSET: f64 = -0.85 / RATIO_DISNST_DT + 25.0;
pub const OVERCURRENT_RECLOSES: u8 = 3;

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
    L: gtsr_gpio::Pin
        + prelude::_embedded_hal_digital_OutputPin
        + prelude::_embedded_hal_digital_ToggleableOutputPin,
    S1: gtsr_gpio::Pin
        + prelude::_embedded_hal_digital_OutputPin
        + prelude::_embedded_hal_digital_ToggleableOutputPin,
    S2: gtsr_gpio::Pin
        + prelude::_embedded_hal_digital_OutputPin
        + prelude::_embedded_hal_digital_ToggleableOutputPin,
    EN1: gtsr_gpio::Pin
        + prelude::_embedded_hal_digital_OutputPin
        + prelude::_embedded_hal_digital_ToggleableOutputPin,
    EN2: gtsr_gpio::Pin
        + prelude::_embedded_hal_digital_OutputPin
        + prelude::_embedded_hal_digital_ToggleableOutputPin,
    DIA: gtsr_gpio::Pin
        + prelude::_embedded_hal_digital_OutputPin
        + prelude::_embedded_hal_digital_ToggleableOutputPin,
> where
    &'a mut L: Default,
    &'a mut S1: Default,
    &'a mut S2: Default,
    &'a mut EN1: Default,
    &'a mut EN2: Default,
    &'a mut DIA: Default,
{
    latch: gtsr_gpio::Signal<'a, L>,
    sel1: gtsr_gpio::Signal<'a, S1>,
    sel2: gtsr_gpio::Signal<'a, S2>,
    en1: gtsr_gpio::Signal<'a, EN1>,
    en2: gtsr_gpio::Signal<'a, EN2>,
    dia_en: gtsr_gpio::Signal<'a, DIA>,
    channel_state: SensorEnablingStatus,

    overcurrent_threshold: [f64; 2],
    min_expected_current: [f64; 2],
    current: [f64; 2],
    status: [LoadSwitchChannelStatus; 2],
    latching: bool,
    overcurrent_retries: [u8; 2],
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

#[allow(deprecated)]
impl<
        L: gtsr_gpio::Pin
            + prelude::_embedded_hal_digital_OutputPin
            + prelude::_embedded_hal_digital_ToggleableOutputPin,
        S1: gtsr_gpio::Pin
            + prelude::_embedded_hal_digital_OutputPin
            + prelude::_embedded_hal_digital_ToggleableOutputPin,
        S2: gtsr_gpio::Pin
            + prelude::_embedded_hal_digital_OutputPin
            + prelude::_embedded_hal_digital_ToggleableOutputPin,
        EN1: gtsr_gpio::Pin
            + prelude::_embedded_hal_digital_OutputPin
            + prelude::_embedded_hal_digital_ToggleableOutputPin,
        EN2: gtsr_gpio::Pin
            + prelude::_embedded_hal_digital_OutputPin
            + prelude::_embedded_hal_digital_ToggleableOutputPin,
        DIA: gtsr_gpio::Pin
            + prelude::_embedded_hal_digital_OutputPin
            + prelude::_embedded_hal_digital_ToggleableOutputPin,
    > LoadSwitch<'static, L, S1, S2, EN1, EN2, DIA>
where
    &'static mut L: Default,
    &'static mut S1: Default,
    &'static mut S2: Default,
    &'static mut EN1: Default,
    &'static mut EN2: Default,
    &'static mut DIA: Default,
{
    pub fn new(
        latch_pin: &'static mut L,
        sel1_pin: Option<&'static mut S1>,
        sel2_pin: Option<&'static mut S2>,
        en1_pin: &'static mut EN1,
        en2_pin: &'static mut EN2,
        dia_pin: &'static mut DIA,
        latching: bool,
        k_sns_value: f64,
        r_sns_value: f64,
        overcurrent_threshold: [f64; 2],
        min_expected_current: [f64; 2],
    ) -> Self {
        let channel_state = if sel1_pin.is_some() && sel2_pin.is_some() {
            SensorEnablingStatus::TempSensing
        } else if sel1_pin.is_some() && sel2_pin.is_none() {
            SensorEnablingStatus::Chan12Sensing
        } else {
            SensorEnablingStatus::Chan1Sensing
        };

        let mut load_switch = LoadSwitch {
            latch: gtsr_gpio::Signal::new(latch_pin),
            sel1: gtsr_gpio::Signal::new(sel1_pin.unwrap_or_default()),
            sel2: gtsr_gpio::Signal::new(sel2_pin.unwrap_or_default()),
            en1: gtsr_gpio::Signal::new(en1_pin),
            en2: gtsr_gpio::Signal::new(en2_pin),
            dia_en: gtsr_gpio::Signal::new(dia_pin),
            channel_state,
            overcurrent_threshold: [
                overcurrent_threshold[LoadSwitchChannel::LoadSwitchCh1 as usize],
                overcurrent_threshold[LoadSwitchChannel::LoadSwitchCh2 as usize],
            ],
            min_expected_current: [
                min_expected_current[LoadSwitchChannel::LoadSwitchCh1 as usize],
                min_expected_current[LoadSwitchChannel::LoadSwitchCh2 as usize],
            ],
            current: [0.0, 0.0],
            status: [
                LoadSwitchChannelStatus::LsStatusNominalOff,
                LoadSwitchChannelStatus::LsStatusNominalOff,
            ],
            latching,
            overcurrent_retries: [0, 0],
            temperature: 0.0,
            current_scale_factor: k_sns_value / r_sns_value,
            temperature_scale_factor: 1000.0 / (r_sns_value * RATIO_DISNST_DT),
            _latch_pin: PhantomData,
            _sel1_pin: PhantomData,
            _sel2_pin: PhantomData,
            _en1_pin: PhantomData,
            _en2_pin: PhantomData,
            _dia_en_pin: PhantomData,
        };

        load_switch.latch.set(latching);

        load_switch.channel_off(LoadSwitchChannel::LoadSwitchCh1);
        load_switch.channel_off(LoadSwitchChannel::LoadSwitchCh2);

        load_switch.dia_en.set_high();

        return load_switch;
    }

    pub fn channel_on(&mut self, channel: LoadSwitchChannel) {
        match channel {
            LoadSwitchChannel::LoadSwitchCh1 => {
                self.en1.set_high();
                self.status[LoadSwitchChannel::LoadSwitchCh1 as usize] =
                    LoadSwitchChannelStatus::LsStatusNominalOn;
            }
            LoadSwitchChannel::LoadSwitchCh2 => {
                self.en2.set_high();
                self.status[LoadSwitchChannel::LoadSwitchCh2 as usize] =
                    LoadSwitchChannelStatus::LsStatusNominalOn;
            }
        }
    }

    pub fn channel_off(&mut self, channel: LoadSwitchChannel) {
        match channel {
            LoadSwitchChannel::LoadSwitchCh1 => {
                self.en1.set_low();
                self.status[LoadSwitchChannel::LoadSwitchCh1 as usize] =
                    LoadSwitchChannelStatus::LsStatusNominalOff;
            }
            LoadSwitchChannel::LoadSwitchCh2 => {
                self.en2.set_low();
                self.status[LoadSwitchChannel::LoadSwitchCh2 as usize] =
                    LoadSwitchChannelStatus::LsStatusNominalOff;
            }
        }
    }

    fn set_sel(&mut self, sel: i8) -> i8 {
        match sel {
            0 | 1 | 2 => {
                if self.channel_state == SensorEnablingStatus::TempSensing {
                    self.sel1.set((sel & 0x3) >> 1 != 0);
                    self.sel2.set(sel & 0x1 != 0);
                } else if self.channel_state == SensorEnablingStatus::Chan12Sensing {
                    self.sel2.set(sel & 0x1 != 0);
                }
            }
            _ => return -1,
        }

        return 0;
    }

    pub fn update_sns(&mut self, sns_voltage: f64, step_sel: bool) {
        let sel = self.sel2.is_set() as u8 | (self.sel1.is_set() as u8) << 1;
        if !self.dia_en.is_set() {
            match sel {
                0 => {
                    self.current[0] = self.current_scale_factor * sns_voltage;

                    if self.current[0] > self.overcurrent_threshold[0] {
                        match self.en1.get_state() {
                            gtsr_gpio::SignalState::Low => {
                                self.status[0] = LoadSwitchChannelStatus::LsStatusFaultOpenCircuit;
                            }
                            gtsr_gpio::SignalState::High => {
                                self.channel_off(LoadSwitchChannel::LoadSwitchCh1);
                                self.status[0] = LoadSwitchChannelStatus::LsStatusFaultOverCurrent;
                            }
                        }
                    } else if self.en1.is_set() && self.current[0] < self.min_expected_current[0] {
                        self.status[0] = LoadSwitchChannelStatus::LsStatusWarningLowCurrent;
                    } else if self.status[0] == LoadSwitchChannelStatus::LsStatusFaultOverCurrent
                        && !self.latching
                        && self.current[0] < self.overcurrent_threshold[0]
                        && self.overcurrent_retries[0] < OVERCURRENT_RECLOSES
                    {
                        self.channel_on(LoadSwitchChannel::LoadSwitchCh1);
                    } else if self.status[0] != LoadSwitchChannelStatus::LsStatusFaultOverCurrent {
                        self.status[0] = if self.en1.is_set() {
                            LoadSwitchChannelStatus::LsStatusNominalOn
                        } else {
                            LoadSwitchChannelStatus::LsStatusNominalOff
                        };
                    }
                }
                1 => {
                    self.current[1] = self.current_scale_factor * sns_voltage;

                    if self.current[1] > self.overcurrent_threshold[1] {
                        match self.en2.get_state() {
                            gtsr_gpio::SignalState::Low => {
                                self.status[1] = LoadSwitchChannelStatus::LsStatusFaultOpenCircuit;
                            }
                            gtsr_gpio::SignalState::High => {
                                self.channel_off(LoadSwitchChannel::LoadSwitchCh2);
                                self.status[1] = LoadSwitchChannelStatus::LsStatusFaultOverCurrent;
                            }
                        }
                    } else if self.en2.is_set() && self.current[1] < self.min_expected_current[1] {
                        self.status[1] = LoadSwitchChannelStatus::LsStatusWarningLowCurrent;
                    } else if self.status[1] == LoadSwitchChannelStatus::LsStatusFaultOverCurrent
                        && !self.latching
                        && self.current[1] < self.overcurrent_threshold[1]
                        && self.overcurrent_retries[1] < OVERCURRENT_RECLOSES
                    {
                        self.channel_on(LoadSwitchChannel::LoadSwitchCh2);
                    } else if self.status[1] != LoadSwitchChannelStatus::LsStatusFaultOverCurrent {
                        self.status[1] = if self.en2.is_set() {
                            LoadSwitchChannelStatus::LsStatusNominalOn
                        } else {
                            LoadSwitchChannelStatus::LsStatusNominalOff
                        };
                    }
                }
                2 => {
                    self.temperature =
                        self.temperature_scale_factor * sns_voltage + TEMP_CONVERSION_OFFSET;
                }
                _ => {
                    self.current[LoadSwitchChannel::LoadSwitchCh1 as usize] =
                        self.current_scale_factor * sns_voltage;
                    self.current[LoadSwitchChannel::LoadSwitchCh2 as usize] =
                        self.current_scale_factor * sns_voltage;
                }
            }
        }

        if step_sel {
            self.set_sel((sel as i8 + 1) % self.channel_state as i8);
        }
    }
}
