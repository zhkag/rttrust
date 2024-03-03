use components::uart::{UartOps, DeviceUart, SerialConfigure};

struct StmUart<'a>{
    uart: &'a mut UsartTypeDef,
}


impl UartOps for StmUart<'_> {
    fn configure(&mut self,  _cfg: &mut SerialConfigure){

    }
    fn control(&mut self,  _cmd: usize, _args: Option<*mut ()>){

    }
    fn putc(&mut self,  c: char){
        self.uart.putc(c);
    }
    fn getc(&mut self) -> u8{
        if let Some(c) =  self.uart.getc(){
            return c as u8;
        }
        0
    }
    
}

use components::drivers::DeviceRegister;

static mut _HW_UART1: Option<DeviceUart> = None;

pub fn hw_usart_init(){
    let stm_uart = StmUart{uart: UsartTypeDef::init()};
    let _hw_uart1 = unsafe {&mut _HW_UART1};
    *_hw_uart1 = Some(DeviceUart::new());
    let _hw_uart1_mut = _hw_uart1.as_mut().unwrap();
    UsartTypeDef::init().usart_init();
    _hw_uart1_mut.register("uart1", stm_uart);
}

use crate::board::board::{RccTypeDef, NvicType, USART1_BASE, GPIOA_BASE};
use crate::drivers::pin::GPIOTypeDef;


#[repr(C)]
pub struct UsartTypeDef {
    sr:u32,         //< USART Status register,                   Address offset: 0x00 */
    dr:u32,         //< USART Data register,                     Address offset: 0x04 */
    brr:u32,        //< USART Baud rate register,                Address offset: 0x08 */
    cr1:u32,        //< USART Control register 1,                Address offset: 0x0C */
    cr2:u32,        //< USART Control register 2,                Address offset: 0x10 */
    cr3:u32,        //< USART Control register 3,                Address offset: 0x14 */
    gtpr:u32,       //< USART Guard time and prescaler register, Address offset: 0x18 */
}

impl UsartTypeDef{
    pub fn init() -> &'static mut Self{
        unsafe {&mut *(USART1_BASE as *mut UsartTypeDef)}
    }
    pub fn putc(&mut self, ch:char)
    {
        self.dr = ch as u32;
        while (self.sr & 0x40) == 0{}     /* 等待字符发送完成 */
    }
    pub fn getc(&mut self) -> Option<char>{
        if self.sr & (1 << 5) != 0 {
            return char::from_u32(self.dr)
        }
        None
    }
    pub fn usart_init(&mut self){
        let rcc = RccTypeDef::init();
        let sclk:u32 = 84;
        let baudrate:u32 = 115200;

        rcc.ahb1enr_0();
        rcc.ahb2enr_4();
        
        let gpioa_base = GPIOTypeDef::init(GPIOA_BASE);

        gpioa_base.set(1 << 9, 2, 0, 1, 1);    /* 串口TX脚 模式设置 */
        gpioa_base.set(1 << 10, 2, 0, 1, 1);    /* 串口RX脚 模式设置 */

        gpioa_base.af_set(1 << 9, 7);    /* TX脚 复用功能选择, 必须设置正确 */
        gpioa_base.af_set(1 << 10, 7);    /* RX脚 复用功能选择, 必须设置正确 */

        let temp:u32 = (sclk * 1000000 + baudrate / 2) / baudrate;              /* 得到USARTDIV@OVER8 = 0, 采用四舍五入计算 */
        /* 波特率设置 */
        self.brr = temp;       /* 波特率设置@OVER8 = 0 */
        self.cr1 = 0;          /* 清零CR1寄存器 */
        self.cr1 |= 0 << 12;   /* 设置M = 0, 选择8位字长 */
        self.cr1 |= 0 << 15;   /* 设置OVER8 = 0, 16倍过采样 */
        self.cr1 |= 1 << 3;    /* 串口发送使能 */
        /* 使能接收中断 */
        self.cr1 |= 1 << 2;    /* 串口接收使能 */
        self.cr1 |= 1 << 5;    /* 接收缓冲区非空中断使能 */
        NvicType::init().nvic_init(3, 3, 37, 2); /* 组2，最低优先级 */
        
        self.cr1 |= 1 << 13;   /* 串口使能 */

    }
}

#[export_name = "USART1_IRQHandler"]
unsafe extern "C" fn usart1_irqhandler() {
    let usart1 = UsartTypeDef::init();
    if usart1.sr & (1 << 5) != 0 {
        usart1.putc(char::from_u32(usart1.dr).unwrap());
    }
}
