use components::pin::{PinOps,DevicePin};
use crate::PinState;
use crate::PinMode;
use kernel::println;
use crate::board::board::AHB1PERIPH_BASE;
const GPIOA_BASE: u32 = AHB1PERIPH_BASE;

#[repr(C)]
pub struct GPIOTypeDef
{
    moder:u32,
    otyper:u32,
    ospeedr:u32,
    pupdr:u32,
    idr:u32,
    odr:u32,
    bsrr:u32,
    lckr:u32,
    afr:[u32;2],
}

impl GPIOTypeDef{
    pub fn init(gpio: u32) -> &'static mut Self{
        unsafe {&mut *(gpio as *mut GPIOTypeDef)}
    }
    pub fn set(&mut self, pinx:u16, mode:u32, otype:u32, ospeed:u32, pupd:u32) {
        let mut pos:u32;
        let mut curpin:u32;
        for pinpos in 0..16 {
            pos = 1 << pinpos;
            curpin = (pinx as u32) & pos;
            if curpin == pos {
                self.moder &= !(3 << (pinpos * 2));
                self.moder |= mode << (pinpos * 2);
                if (mode == 1) || (mode == 2) {
                    self.ospeedr &= !(3 << (pinpos * 2));
                    self.ospeedr |= ospeed << (pinpos * 2);
                    self.otyper &= !(1 << pinpos);
                    self.otyper |= otype << pinpos;
                }
                self.pupdr &= !(3 << (pinpos * 2));
                self.pupdr |= pupd << (pinpos * 2);
            }
        }
    }

    pub fn pin_set(&mut self, pinx:u32, status:PinState)
    {
        if status.into() {
            self.bsrr |= pinx;
        }
        else {
            self.bsrr |= pinx << 16;
        }
    }

    pub fn af_set(&mut self, pinx:u32, afx:u32)
    {
        let mut pos:u32;
        let mut curpin: u32;
        for pinpos in 0..16 {
            pos = 1 << pinpos;      /* 一个个位检查 */
            curpin = pinx & pos;    /* 检查引脚是否要设置 */

            if curpin == pos{
                self.afr[pinpos >> 3] &= !(0x0F << ((pinpos & 0x07) * 4));
                self.afr[pinpos >> 3] |= afx << ((pinpos & 0x07) * 4);
            }
        }
    }
}

struct StmPin;

impl StmPin{
    #[inline]
    fn num(port:usize, no:usize) -> usize{
        ((port&0xF) << 4) | (no&0xF)
    }
    #[inline]
    fn port(pin:usize) -> u8{
        ((pin >> 4) & 0xF) as u8
    }
    #[inline]
    fn no(pin:usize) -> u8{
        (pin & 0xF) as u8
    }
    #[inline]
    fn st_port(pin:usize) -> &'static mut GPIOTypeDef {
        unsafe { &mut *((GPIOA_BASE + (0x400 * Self::port(pin) as u32)) as *mut GPIOTypeDef)}
    }
    #[inline]
    fn st_pin(pin:usize) -> u16 {
        1 << Self::no(pin)
    }

}

impl PinOps for StmPin {
    fn pin_mode(&mut self,  pin: usize, pin_mode: PinMode){
        let gpio_port = Self::st_port(pin);
        let gpio_pin = Self::st_pin(pin) as u32;
        let mode:u32;
        let mut otype:u32 = 0;
        let ospeed:u32 = 3;
        let pupd:u32;
        match pin_mode {
            PinMode::OUTPUT => {mode = 1; otype = 0; pupd = 0;},
            PinMode::INPUT => {mode = 0; pupd = 0;},
            PinMode::InputPullup => {mode = 0; pupd = 1;},
            PinMode::InputPulldown => {mode = 0; pupd = 2;},
            PinMode::OutputOd => {mode = 1; otype = 1; pupd = 0;},
        }
        gpio_port.set(gpio_pin as u16, mode, otype, ospeed, pupd);
    }
    fn pin_write(&mut self,  pin: usize, value: PinState){
        let gpio_port = Self::st_port(pin);
        let gpio_pin = Self::st_pin(pin) as u32;
        gpio_port.pin_set(gpio_pin, value);
    }
    fn pin_read(&mut self,  pin: usize) -> PinState{
        let gpio_port = Self::st_port(pin);
        let gpio_pin = Self::st_pin(pin) as u32;
        if gpio_port.idr & gpio_pin > 0 {
            return PinState::HIGH;
        }else{
            return PinState::LOW;
        }
    }
    fn pin_detach_irq(&mut self,  _pin: usize){
        println!("pin_detach_irq");

    }
    fn pin_irq_enable(&mut self,  _pin: usize, _enabled: u8){
        println!("pin_irq_enable");
    }
    fn pin_get(&mut self, name:&str) -> usize{
        let name_bytes = name.as_bytes();
        let mut port_num:usize = 0;
        let mut pin_num:usize = 0;
        let name_bytes0 = *name_bytes.get(0).unwrap();
        let name_bytes1 = *name_bytes.get(1).unwrap();
        let name_bytes2 = *name_bytes.get(2).unwrap();
        if name_bytes0 != b'P' || name_bytes2 != b'.' {
            return 0;
        }
        if (name_bytes1 >= b'A') && (name_bytes1 <= b'Z')
        {
            port_num = (name_bytes1 - b'A') as usize;
        }
        for i in 3..name.len() {
            pin_num *= 10;
            pin_num += (*name_bytes.get(i).unwrap() - b'0') as usize;
        }
        Self::num(port_num, pin_num)
    }
}

const RCC_BASE: u32 = AHB1PERIPH_BASE + 0x3800;
const AHB1ENR: u32 = RCC_BASE + 0x30;
use components::drivers::DeviceRegister;
use kernel::macros::init_export;
#[init_export("1")]
fn device_pin() {
    let ahb1enr_ptr: *mut u32 = AHB1ENR as *mut u32;
    unsafe {
        let ahb1enr = &mut *ahb1enr_ptr;
        *ahb1enr |= 1 << 5;
    }
    let stm_device_pin = StmPin{};
    DevicePin::new().register("pin",stm_device_pin);
}

