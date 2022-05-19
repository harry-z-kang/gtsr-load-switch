use tm4c123x_hal as hal;

use self::hal::gpio;

pub trait GpioParts {}

impl GpioParts for gpio::gpioa::Parts {}
impl GpioParts for gpio::gpiob::Parts {}
impl GpioParts for gpio::gpioc::Parts {}
impl GpioParts for gpio::gpiod::Parts {}
impl GpioParts for gpio::gpioe::Parts {}
impl GpioParts for gpio::gpiof::Parts {}

pub struct Signal<T: GpioParts> {}

impl<T: GpioParts> Signal<T> {
    pub fn new(&self, parts: T, pin: u8) -> Self {
        match pin {
            0 => parts.pa0.into_push_pull_output(),
            1 => parts.pa1.into_push_pull_output(),
            2 => parts.pa2.into_push_pull_output(),
            3 => parts.pa3.into_push_pull_output(),
            4 => parts.pa4.into_push_pull_output(),
            5 => parts.pa5.into_push_pull_output(),
            6 => parts.pa6.into_push_pull_output(),
            7 => parts.pa7.into_push_pull_output(),
            _ => panic!("Invalid pin number"),
        }
    }
}
