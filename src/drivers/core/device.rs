#[repr(C)]
#[allow(dead_code)]
pub enum DeviceClassType
{
    Char = 0,                           //< character device */
    Block,                              //< block device */
    NetIf,                              //< net interface */
    MTD,                                //< memory device */
    CAN,                                //< CAN device */
    RTC,                                //< RTC device */
    Sound,                              //< Sound device */
    Graphic,                            //< Graphic device */
    I2CBUS,                             //< I2C bus device */
    USBDevice,                          //< USB slave device */
    USBHost,                            //< USB host bus */
    USBOTG,                             //< USB OTG bus */
    SPIBUS,                             //< SPI bus device */
    SPIDevice,                          //< SPI device */
    SDIO,                               //< SDIO bus device */
    PM,                                 //< PM pseudo device */
    Pipe,                               //< Pipe device */
    Portal,                             //< Portal device */
    Timer,                              //< Timer device */
    Miscellaneous,                      //< Miscellaneous device */
    Sensor,                             //< Sensor device */
    Touch,                              //< Touch device */
    PHY,                                //< PHY device */
    Security,                           //< Security device */
    WLAN,                               //< WLAN device */
    Pin,                                //< Pin device */
    ADC,                                //< ADC device */
    DAC,                                //< DAC device */
    WDT,                                //< WDT device */
    PWM,                                //< PWM device */
    Bus,                                //< Bus device */
    Unknown                             //< unknown device */
}

#[allow(dead_code)]
#[repr(C)]
pub struct Device
{
    name:String,
    r#type:DeviceClassType,                     //< device type */
    flag:u16,                     //< device flag */
    open_flag:u16,                //< device open flag */
    ref_count:u8,                //< reference count */
    device_id:u8,                //< 0 - 255 */
    user_data: *mut (),                //< device private data */
}

use crate::Any;
use crate::Box;
use crate::system::System;
use alloc::string::{String, ToString};

pub trait DeviceOps {
    fn name(&self) -> &str {""}
    fn as_any(&mut self) -> &mut dyn Any;
    fn rx_indicate(&mut self, _size:usize) -> isize { 0 }
    fn tx_complete(&mut self, _buffer: *mut ()) -> isize { 0 }
    fn init(&mut self) -> isize { 0 }
    fn open(&mut self, _oflag:u16) -> isize { 0 }
    fn close(&mut self) -> isize { 0 }
    fn read(&mut self, _pos:isize, _buffer: Option<*mut ()>, _size:usize) -> isize { 0 }
    fn write(&mut self, _pos:isize, _buffer: Option<*const ()>, _size:usize) -> isize { 0 }
    fn control(&mut self, _cmd:usize, _args: Option<*mut ()>) -> isize { 0 }
}

impl System {
    pub fn device_list_mut(&mut self) -> &mut crate::BTreeMap<alloc::string::String, Box<dyn DeviceOps>>{
        &mut self.device_list
    }
    pub fn device_register(&mut self,item: impl DeviceOps + 'static){
        self.device_list.insert(item.name().to_string(), Box::new(item));
    }
}

impl Device {
    pub fn new() -> Self{
        let derive = Self{
            name:"".to_string(),
            r#type: DeviceClassType::Unknown,
            flag: 0,
            open_flag:0,
            ref_count:0,
            device_id:0,
            user_data:core::ptr::null_mut(),
        };
        derive
    }
    pub fn init(&mut self, name:&str, r#type: DeviceClassType){
        self.r#type = r#type;
        self.name = name.to_string();
    }
    pub fn name(&self) -> &str{
        &self.name
    }
}

pub trait DeviceRegister<T> {
    fn register(&mut self, name:&str, ops:T);
}

#[macro_export]
macro_rules! derive_find {
    ($derive:literal) => {
        $crate::system!(device_list_mut()).get_mut($derive)
    };
}
