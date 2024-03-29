use kernel::drivers::i2c::dev::DeviceI2cBus;
use kernel::system;
use kernel::String;
use crate::pin_write;
use crate::pin_read;
use crate::pin_mode;
use crate::PinState;
use crate::PinMode;

use kernel::drivers::i2c::bitops::{I2cBit,I2cBitOps};


struct Stm32SoftI2cConfig
{
    scl:u8,
    sda:u8,
    bus_name:String,
}


struct Stm32I2c
{
    parent:I2cBit,
    config:Stm32SoftI2cConfig,
}

impl Stm32I2c {
    pub fn new() -> Stm32I2c {
        Stm32I2c{
            parent:I2cBit::init(None, 1, 100),
            config:Stm32SoftI2cConfig{
                scl:81,
                sda:80,
                bus_name:String::from("i2c"),
            }
        }
    }
    fn gpio_init(&mut self) {
        pin_mode(self.config.scl.into(), PinMode::OutputOd);
        pin_mode(self.config.sda.into(), PinMode::OutputOd);
        pin_write(self.config.scl.into(), PinState::HIGH);
        pin_write(self.config.sda.into(), PinState::HIGH);
    }
}

impl I2cBitOps for Stm32I2c {
    fn set_sda(&mut self, state:usize) {
        if state != 0 {
            pin_write(self.config.sda.into(), PinState::HIGH);
        }else {
            pin_write(self.config.sda.into(), PinState::LOW);
        }
    }
    fn set_scl(&mut self, state:usize) {
        if state != 0 {
            pin_write(self.config.scl.into(), PinState::HIGH);
        }else {
            pin_write(self.config.scl.into(), PinState::LOW);
        }
    }
    fn get_sda(&mut self) -> usize {
        pin_read(self.config.sda.into()) as usize
    }
    fn get_scl(&mut self) -> usize {
        pin_read(self.config.scl.into()) as usize
    }
    fn udelay(&self, us:usize) {
        system!(bsp().unwrap().us_delay(us));
    }
}

use kernel::macros::init_export;
use components::drivers::DeviceRegister;
#[init_export("2")]
fn device_i2c() {
    let mut stm_i2c = Stm32I2c::new();
    stm_i2c.gpio_init();
    I2cBit::new().register("i2c2", stm_i2c);
}

