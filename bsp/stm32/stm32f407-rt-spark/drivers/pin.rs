use components::PinOps;
use components::DevicePin;

use kernel::println;

struct StmPin
{
}

impl PinOps for StmPin {
    fn pin_mode(&mut self,  _pin: isize, _mode: u8){
        println!("pin_mode");
    }
    fn pin_write(&mut self,  _pin: isize, _value: u8){
        println!("pin_write");

    }
    fn pin_read(&mut self,  _pin: isize){
        println!("pin_read");

    }
    fn pin_detach_irq(&mut self,  _pin: isize){
        println!("pin_detach_irq");

    }
    fn pin_irq_enable(&mut self,  _pin: isize, _enabled: u8){
        println!("pin_irq_enable");
    }
}

use kernel::macros::init_export;
#[init_export("1")]
fn device_pin() {
    let mut stm_pin = StmPin{};
    DevicePin::register("pin",&mut stm_pin);
}

