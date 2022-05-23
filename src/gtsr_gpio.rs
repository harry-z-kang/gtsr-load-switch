use tm4c123x_hal as hal;

use self::hal::gpio;

pub trait Pin {}

impl Pin for gpio::gpioa::PA0<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioa::PA1<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioa::PA2<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioa::PA3<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioa::PA4<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioa::PA5<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioa::PA6<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioa::PA7<gpio::Output<gpio::PushPull>> {}

impl Pin for gpio::gpiob::PB0<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiob::PB1<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiob::PB2<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiob::PB3<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiob::PB4<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiob::PB5<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiob::PB6<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiob::PB7<gpio::Output<gpio::PushPull>> {}

// impl Pin for gpio::gpioc::PC0<gpio::Output<gpio::PushPull>> {}
// impl Pin for gpio::gpioc::PC1<gpio::Output<gpio::PushPull>> {}
// impl Pin for gpio::gpioc::PC2<gpio::Output<gpio::PushPull>> {}
// impl Pin for gpio::gpioc::PC3<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioc::PC4<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioc::PC5<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioc::PC6<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioc::PC7<gpio::Output<gpio::PushPull>> {}

impl Pin for gpio::gpiod::PD0<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiod::PD1<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiod::PD2<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiod::PD3<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiod::PD4<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiod::PD5<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiod::PD6<gpio::Output<gpio::PushPull>> {}
// impl Pin for gpio::gpiod::PD7<gpio::Output<gpio::PushPull>> {}

impl Pin for gpio::gpioe::PE0<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioe::PE1<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioe::PE2<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioe::PE3<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioe::PE4<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioe::PE5<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioe::PE6<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpioe::PE7<gpio::Output<gpio::PushPull>> {}

// impl Pin for gpio::gpiof::PF0<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiof::PF1<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiof::PF2<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiof::PF3<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiof::PF4<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiof::PF5<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiof::PF6<gpio::Output<gpio::PushPull>> {}
impl Pin for gpio::gpiof::PF7<gpio::Output<gpio::PushPull>> {}

pub struct Signal<T: Pin> {
    pin: T,
}
