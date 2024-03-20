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
    r#type:DeviceClassType,                     //< device type */
    flag:u16,                     //< device flag */
    open_flag:u16,                //< device open flag */
    ref_count:u8,                //< reference count */
    device_id:u8,                //< 0 - 255 */
    user_data: *mut (),                //< device private data */
}

use crate::Box;
use crate::system::System;
use alloc::string::ToString;

pub trait DeviceOps {
    fn name(&self) -> &str {""}
    fn device_self(&mut self) -> Option<DeviceSelf> {None}
    fn rx_indicate(&mut self, _size:usize) -> isize { 0 }
    fn tx_complete(&mut self, _buffer: *mut ()) -> isize { 0 }
    fn init(&mut self) -> isize { 0 }
    fn open(&mut self, _oflag:u16) -> isize { 0 }
    fn close(&mut self) -> isize { 0 }
    fn read(&mut self, _pos:isize, _buffer: Option<*mut ()>, _size:usize) -> isize { 0 }
    fn write(&mut self, _pos:isize, _buffer: Option<*const ()>, _size:usize) -> isize { 0 }
    fn control(&mut self, _cmd:usize, _args: Option<*mut ()>) -> isize { 0 }
}

impl System<'_> {
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
            r#type: DeviceClassType::Unknown,
            flag: 0,
            open_flag:0,
            ref_count:0,
            device_id:0,
            user_data:core::ptr::null_mut(),
        };
        derive
    }
    pub fn init(&mut self, r#type: DeviceClassType){
        self.r#type = r#type;
    }
}

pub trait DeviceRegister<T> {
    fn register(&mut self, name:&str, ops:T);
}

use crate::drivers::pin::DevicePin;
use crate::drivers::serial::uart::DeviceUart;
pub enum DeviceSelf<'a> {
    Pin(&'a mut DevicePin),
    Uart(&'a mut DeviceUart),
}
