const PERIPH_BASE: u32 = 0x40000000;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x00010000;
const USART1_BASE: u32 = APB2PERIPH_BASE + 0x1000;


#[repr(C)]
struct UsartTypeDef {
    sr:u32,         //< USART Status register,                   Address offset: 0x00 */
    dr:u32,         //< USART Data register,                     Address offset: 0x04 */
    brr:u32,        //< USART Baud rate register,                Address offset: 0x08 */
    cr1:u32,        //< USART Control register 1,                Address offset: 0x0C */
    cr2:u32,        //< USART Control register 2,                Address offset: 0x10 */
    cr3:u32,        //< USART Control register 3,                Address offset: 0x14 */
    gtpr:u32,       //< USART Guard time and prescaler register, Address offset: 0x18 */
}

pub fn putc(ch:char)
{
    let usart1 = unsafe {&mut *(USART1_BASE as *mut UsartTypeDef)};
    usart1.dr = ch as u32;
    while (usart1.sr & 0x40) == 0{}     /* 等待字符发送完成 */
}


pub trait BoardTrait {
    fn init(&self);
}

impl crate::system::System {
    pub fn board(&self) -> &dyn BoardTrait{
        unsafe { &*(self.board.unwrap())}
    }
    pub fn board_trait_init(&mut self,item: *mut dyn BoardTrait) {
        self.board = Some(item);
    }
}
