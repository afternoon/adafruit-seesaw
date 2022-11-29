#[macro_export]
macro_rules! seesaw_device {
    ($device:ident, default_addr: $default_addr:expr) => {
        /// $device
        #[derive(Debug)]
        pub struct $device<M>(u8, M);

        impl<D: $crate::driver::Driver> $crate::device::Device<D> for $device<D> {
            type Error = $crate::SeesawError<D::I2cError>;

            fn addr(&self) -> u8 {
                self.0
            }

            fn driver<'a>(&'a mut self) -> &'a mut D {
                &mut self.1
            }

            fn new(addr: u8, driver: D) -> Self {
                Self(addr, driver)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_device_encoder_module {
    ($device:ident, button_pin: $button_pin:expr) => {
        impl<D: $crate::driver::Driver> $crate::EncoderModule<D> for $device<D> {
            const ENCODER_BTN_PIN: u8 = $button_pin;
        }
    };
}

#[macro_export]
macro_rules! impl_device_gpio_module {
    ($device:ident) => {
        impl<D: $crate::driver::Driver> $crate::GpioModule<D> for $device<D> {}
    };
}

#[macro_export]
macro_rules! impl_device_neopixel_module {
    ($device:ident, num_leds: $num_leds:expr, pin: $pin:expr) => {
        impl<D: $crate::driver::Driver> $crate::NeopixelModule<D> for $device<D> {
            const N_LEDS: u16 = $num_leds;
            const PIN: u8 = $pin;
        }
    };
}
