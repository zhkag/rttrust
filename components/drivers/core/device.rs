#[repr(C)]
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

struct Device
{
    pub(super) parent:Object,
    r#type:ObjectClassType,                     //< device type */
    flag:u16,                     //< device flag */
    open_flag:u16,                //< device open flag */
    ref_count:u8,                //< reference count */
    device_id:u8,                //< 0 - 255 */
    user_data: *mut (),                //< device private data */
}

trait DeviceOps {
    fn rx_indicate(&mut self, size:usize) -> isize;
    fn tx_complete(&mut self, buffer: *mut ()) -> isize;
    fn init(&mut self) -> isize;
    fn open(&mut self, oflag:u16) -> isize;
    fn close(&mut self) -> isize;
    fn read(&mut self, pos:isize, buffer: *mut (), size:usize) -> isize;
    fn write(&mut self, pos:isize, buffer: *const (), size:usize) -> isize;
    fn control(&mut self,size:usize) -> isize;
}
