const SCS_BASE: u32 = 0xE000E000;
const NVIC_BASE: u32 = SCS_BASE + 0x0100;
const SCB_BASE: u32 = SCS_BASE + 0x0D00;
const PERIPH_BASE: u32 = 0x40000000;
const APB1PERIPH_BASE: u32 = PERIPH_BASE;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x00010000;
const AHB1PERIPH_BASE: u32 = PERIPH_BASE + 0x00020000;
const PWR_BASE: u32 = APB1PERIPH_BASE + 0x7000;
pub const USART1_BASE: u32 = APB2PERIPH_BASE + 0x1000;
const RCC_BASE: u32 = AHB1PERIPH_BASE + 0x3800;
pub const GPIOA_BASE: u32 = AHB1PERIPH_BASE + 0x0000;
#[allow(dead_code)]
const GPIOF_BASE: u32 = AHB1PERIPH_BASE + 0x1400;
const FLASH_R_BASE: u32 = AHB1PERIPH_BASE + 0x3C00;
#[allow(dead_code)]
const AHB1ENR: u32 = RCC_BASE + 0x30;
const SYS_TICK_BASE: u32 = SCS_BASE + 0x0010;

#[repr(C)]
pub struct NvicType
{
    iser:[u32;8],              //< Offset: 0x000 (R/W)  Interrupt Set Enable Register */
    reserved0:[u32;24],
    icer:[u32;8],              //< Offset: 0x080 (R/W)  Interrupt Clear Enable Register */
    rserved1:[u32;24],
    ispr:[u32;8],              //< Offset: 0x100 (R/W)  Interrupt Set Pending Register */
    reserved2:[u32;24],
    icpr:[u32;8],              //< Offset: 0x180 (R/W)  Interrupt Clear Pending Register */
    reserved3:[u32;24],
    iabr:[u32;8],              //< Offset: 0x200 (R/W)  Interrupt Active bit Register */
    reserved4:[u32;56],
    ip:[u8;240],              //< Offset: 0x300 (R/W)  Interrupt Priority Register (8Bit wide) */
    reserved5:[u32;644],
    stir:u32,                  //< Offset: 0xE00 ( /W)  Software Trigger Interrupt Register */
}

#[repr(C)]
struct ScbType
{
    cpuid:u32,                  //< Offset: 0x000 (R/ )  CPUID Base Register */
    icsr:u32,                   //< Offset: 0x004 (R/W)  Interrupt Control and State Register */
    vtor:u32,                   //< Offset: 0x008 (R/W)  Vector Table Offset Register */
    aircr:u32,                  //< Offset: 0x00C (R/W)  Application Interrupt and Reset Control Register */
    scr:u32,                    //< Offset: 0x010 (R/W)  System Control Register */
    ccr:u32,                    //< Offset: 0x014 (R/W)  Configuration Control Register */
    shp:[u8;12],               //< Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15) */
    shcsr:u32,                  //< Offset: 0x024 (R/W)  System Handler Control and State Register */
    cfsr:u32,                   //< Offset: 0x028 (R/W)  Configurable Fault Status Register */
    hfsr:u32,                   //< Offset: 0x02C (R/W)  HardFault Status Register */
    dfsr:u32,                   //< Offset: 0x030 (R/W)  Debug Fault Status Register */
    mmfar:u32,                  //< Offset: 0x034 (R/W)  MemManage Fault Address Register */
    bfar:u32,                   //< Offset: 0x038 (R/W)  BusFault Address Register */
    afsr:u32,                   //< Offset: 0x03C (R/W)  Auxiliary Fault Status Register */
    pfr:[u32;2],                //< Offset: 0x040 (R/ )  Processor Feature Register */
    dfr:u32,                    //< Offset: 0x048 (R/ )  Debug Feature Register */
    adr:u32,                    //< Offset: 0x04C (R/ )  Auxiliary Feature Register */
    mmfr:[u32;4],               //< Offset: 0x050 (R/ )  Memory Model Feature Register */
    isar:[u32;5],               //< Offset: 0x060 (R/ )  Instruction Set Attributes Register */
    reserved0:[u32;5],
    cpacr:u32,                  //< Offset: 0x088 (R/W)  Coprocessor Access Control Register */
}

impl ScbType {
    fn init() -> &'static mut Self{
        unsafe {&mut *(SCB_BASE as *mut ScbType)}
    }
    fn nvic_priority_group_config(&mut self, group:u8)
    {
        let mut temp:u32;
        let mut temp1:u32;
        temp1 = (!group as u32) & 0x07;/* 取后三位 */
        temp1 <<= 8;
        temp = self.aircr;      /* 读取先前的设置 */
        temp &= 0x0000F8FF;     /* 清空先前分组 */
        temp |= 0x05FA0000;     /* 写入钥匙 */
        temp |= temp1;
        self.aircr = temp;      /* 设置分组 */
    }
}

impl NvicType{
    pub fn init() -> &'static mut Self{
        unsafe {&mut *(NVIC_BASE as *mut NvicType)}
    }
    pub fn nvic_init(&mut self, pprio:u8, sprio:u8, ch:u8, group:u8)
    {
        let mut temp:u32;
        ScbType::init().nvic_priority_group_config(group);  /* 设置分组 */
        temp = (pprio << (4 - group)) as u32;
        temp |= sprio as u32 & (0x0f >> group);
        temp &= 0xf;                            /* 取低四位 */
        self.iser[(ch / 32) as usize] |= (1 << (ch % 32)) as u32;  /* 使能中断位(要清除的话,设置ICER对应位为1即可) */
        self.ip[ch as usize] |= (temp << 4) as u8;              /* 设置响应优先级和抢断优先级 */
    }
}

#[repr(C)]
struct SysTickType {
    ctrl:u32,
    load:u32,
    val:u32,
    valib:u32,
}

impl SysTickType{
    fn init() -> &'static mut Self{
        unsafe {&mut *(SYS_TICK_BASE as *mut SysTickType)}
    }
    fn systick_init(&mut self) {
        self.ctrl |= 1 << 2;
        self.ctrl |= 1 << 0;                 /* 使能Systick */
        self.load = 0x0fffffff;              /* 注意systick计数器24位，所以这里设置最大重装载值 */
        self.ctrl |= 1 << 1;                 /* 开启SYSTICK中断 */
        self.load = 100000;                  /* 每1/delay_ostickspersec秒中断一次 */
    }
}

#[repr(C)]
pub struct RccTypeDef {
    cr:u32,            //< RCC clock control register,                                  Address offset: 0x00 */
    pllcfgr:u32,       //< RCC PLL configuration register,                              Address offset: 0x04 */
    cfgr:u32,          //< RCC clock configuration register,                            Address offset: 0x08 */
    cir:u32,           //< RCC clock interrupt register,                                Address offset: 0x0C */
    ahb1rstr:u32,      //< RCC AHB1 peripheral reset register,                          Address offset: 0x10 */
    ahb2rstr:u32,      //< RCC AHB2 peripheral reset register,                          Address offset: 0x14 */
    ahb3rstr:u32,      //< RCC AHB3 peripheral reset register,                          Address offset: 0x18 */
    reserved0:u32,     //< Reserved, 0x1C                                                                    */
    apb1rstr:u32,      //< RCC APB1 peripheral reset register,                          Address offset: 0x20 */
    apb2rstr:u32,      //< RCC APB2 peripheral reset register,                          Address offset: 0x24 */
    reserved1:[u32;2],  //< Reserved, 0x28-0x2C                                                               */
    ahb1enr:u32,       //< RCC AHB1 peripheral clock register,                          Address offset: 0x30 */
    ahb2enr:u32,       //< RCC AHB2 peripheral clock register,                          Address offset: 0x34 */
    ahb3enr:u32,       //< RCC AHB3 peripheral clock register,                          Address offset: 0x38 */
    reserved2:u32,     //< Reserved, 0x3C                                                                    */
    apb1enr:u32,       //< RCC APB1 peripheral clock enable register,                   Address offset: 0x40 */
    apb2enr:u32,       //< RCC APB2 peripheral clock enable register,                   Address offset: 0x44 */
    reserved3:[u32;2],  //< Reserved, 0x48-0x4C                                                               */
    ahb1lpenr:u32,     //< RCC AHB1 peripheral clock enable in low power mode register, Address offset: 0x50 */
    ahb2lpenr:u32,     //< RCC AHB2 peripheral clock enable in low power mode register, Address offset: 0x54 */
    ahb3lpenr:u32,     //< RCC AHB3 peripheral clock enable in low power mode register, Address offset: 0x58 */
    reserved4:u32,     //< Reserved, 0x5C                                                                    */
    apb1lpenr:u32,     //< RCC APB1 peripheral clock enable in low power mode register, Address offset: 0x60 */
    apb2lpenr:u32,     //< RCC APB2 peripheral clock enable in low power mode register, Address offset: 0x64 */
    reserved5:[u32;2],  //< Reserved, 0x68-0x6C                                                               */
    bdcr:u32,          //< RCC Backup domain control register,                          Address offset: 0x70 */
    csr:u32,           //< RCC clock control & status register,                         Address offset: 0x74 */
    reserved6:[u32;2],  //< Reserved, 0x78-0x7C                                                               */
    sscgr:u32,         //< RCC spread spectrum clock generation register,               Address offset: 0x80 */
    plli2scfgr:u32,    //< RCC PLLI2S configuration register,                           Address offset: 0x84 */
}

#[repr(C)]
struct FlashTypeDef {
    acr:u32,       //< FLASH access control register,   Address offset: 0x00 */
    keyr:u32,      //< FLASH key register,              Address offset: 0x04 */
    optkeyr:u32,   //< FLASH option key register,       Address offset: 0x08 */
    sr:u32,        //< FLASH status register,           Address offset: 0x0C */
    cr:u32,        //< FLASH control register,          Address offset: 0x10 */
    optcr:u32,     //< FLASH option control register ,  Address offset: 0x14 */
    optcr1:u32,    //< FLASH option control register 1, Address offset: 0x18 */
}

#[repr(C)]
struct PwrTypeDef {
    cr:u32,   //< PWR power control register,        Address offset: 0x00 */
    csr:u32,  //< PWR power control/status register, Address offset: 0x04 */
}

impl FlashTypeDef{
    fn init() -> &'static mut Self{
        unsafe {&mut *(FLASH_R_BASE as *mut FlashTypeDef)}
    }
}

impl PwrTypeDef{
    fn init() -> &'static mut Self{
        unsafe {&mut *(PWR_BASE as *mut PwrTypeDef)}
    }
}

impl RccTypeDef{
    pub fn init() -> &'static mut Self{
        unsafe {&mut *(RCC_BASE as *mut RccTypeDef)}
    }
    pub fn ahb1enr_0(&mut self){
        self.ahb1enr |= 1 << 0;      /* 使能串口TX脚时钟 */
        self.ahb1enr |= 1 << 0;      /* 使能串口RX脚时钟 */
    }
    pub fn ahb2enr_4(&mut self){
        self.apb2enr |= 1 << 4;      /* 使能串口时钟 */
    }
    fn clock_init(&mut self){
        self.cr = 0x00000001;           /* 设置HISON, 开启内部高速RC振荡，其他位全清零 */
        self.cfgr = 0x00000000;         /* CFGR清零 */
        self.pllcfgr = 0x00000000;      /* PLLCFGR清零 */
        self.cir = 0x00000000;          /* CIR清零 */
        self.sys_clock_set(336, 8, 2, 7);
    }

    fn sys_clock_set(&mut self, plln:u32, pllm:u32, pllp:u32, pllq:u32) -> u8{
        let flash = FlashTypeDef::init();
        let pwr = PwrTypeDef::init();
        let mut retry:u32 = 0;
        let mut retval:u8 = 0;
        let mut swsval:u8 = 0;
    
        self.cr |= 1 << 16; /* HSEON = 1, 开启HSE */
    
        while ((self.cr & (1 << 17)) == 0) && (retry < 0x7FFF) {
            retry += 1;        /* 等待HSE RDY */
        }
    
        if retry == 0x7FFF {
            retval = 1;     /* HSE无法就绪 */
            return retval;
        }
    
        self.apb1enr |= 1 << 28;                /* 电源接口时钟使能 */
        pwr.cr |= 3 << 14;                     /* 高性能模式,时钟可到168Mhz */
        
        self.pllcfgr |= 0x3F & pllm;            /* 设置主PLL预分频系数,  PLLM[5:0]: 2~63 */
        self.pllcfgr |= plln << 6;              /* 设置主PLL倍频系数,    PLLN[8:0]: 192~432 */
        self.pllcfgr |= ((pllp >> 1) - 1) << 16;/* 设置主PLL的p分频系数, PLLP[1:0]: 0~3, 代表2~8分频 */
        self.pllcfgr |= pllq << 24;             /* 设置主PLL的q分频系数, PLLQ[3:0]: 2~15 */
        self.pllcfgr |= 1 << 22;                /* 设置主PLL的时钟源来自HSE */
    
        self.cfgr |= 0 << 4;                    /* HPRE[3:0]  = 0, AHB  不分频, rcc_hclk1/2/3 = pll_p_ck */
        self.cfgr |= 5 << 10;                   /* PPRE1[2:0] = 5, APB1 4分频   rcc_pclk1 = pll_p_ck / 4 */
        self.cfgr |= 4 << 13;                   /* PPRE2[2:0] = 4, APB2 2分频   rcc_pclk2 = pll_p_ck / 2 */
    
        self.cr |= 1 << 24;                     /* 打开主PLL */
    
        retry = 0;
        while (self.cr & (1 << 25)) == 0 {      /* 等待PLL准备好 */
            retry += 1;
            if retry > 0x1FFFFF {
                retval = 2;                     /* 主PLL无法就绪 */
                break;
            }
        }
    
        flash.acr |= 1 << 8;                   /* 指令预取使能 */
        flash.acr |= 1 << 9;                   /* 指令cache使能 */
        flash.acr |= 1 << 10;                  /* 数据cache使能 */
        flash.acr |= 5 << 0;                   /* 5个CPU等待周期 */
        
        self.cfgr |= 2 << 0;                    /* 选择主PLL作为系统时钟 */
        
        retry = 0;
        while swsval != 3                     /* 等待成功将系统时钟源切换为pll_p_ck */
        {
            swsval = (self.cfgr as u8 & 0x0C) >> 2;   /* 获取SWS[1:0]的状态, 判断是否切换成功 */
            retry += 1;
            if retry > 0x1FFFFF
            {
                retval = 4; /* 无法切换时钟 */
                break;
            }
        }
        return retval;
    }
}


use kernel::BspTrait;
use crate::drivers::uart::hw_usart_init;
struct Board;

use components::uart::DeviceUart;

impl BspTrait for Board {
    fn init(&self){
        RccTypeDef::init().clock_init();
        SysTickType::init().systick_init();
        hw_usart_init();
    }
    fn putc(&self,  c: char) {
        if let Some(pin) = DeviceUart::find("uart1"){
            pin.ops().putc(c);
        }
    }
}

#[kernel::macros::init_export("0.0")]
fn board_init() {
    let mut board = Board{};
    let system = kernel::system!();
    system.bsp_trait_init(&mut board);
}
