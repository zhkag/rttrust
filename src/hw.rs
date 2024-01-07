#[derive(Copy, Clone)]
pub struct HardWare {
    
}

const PERIPH_BASE: u32 = 0x40000000;
const APB1PERIPH_BASE: u32 = PERIPH_BASE;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x00010000;
const AHB1PERIPH_BASE: u32 = PERIPH_BASE + 0x00020000;
const PWR_BASE: u32 = APB1PERIPH_BASE + 0x7000;
const USART1_BASE: u32 = APB2PERIPH_BASE + 0x1000;
const RCC_BASE: u32 = AHB1PERIPH_BASE + 0x3800;
const GPIOA_BASE: u32 = AHB1PERIPH_BASE + 0x0000;
const GPIOF_BASE: u32 = AHB1PERIPH_BASE + 0x1400;
const FLASH_R_BASE: u32 = AHB1PERIPH_BASE + 0x3C00;
const AHB1ENR: u32 = RCC_BASE + 0x30;


const SCS_BASE: u32 = 0xE000E000;
const SYS_TICK_BASE: u32 = SCS_BASE + 0x0010;


#[repr(C)]
struct ExceptionStackFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    psr: u32,
}
#[repr(C)]
struct StackFrame {
    flag: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    exception_stack_frame: ExceptionStackFrame,
}

#[repr(C)]
struct SysTickType {
    ctrl:u32,
    load:u32,
    val:u32,
    valib:u32,
}

fn systick_init() {
    let systick = unsafe {&mut *(SYS_TICK_BASE as *mut SysTickType)};
    systick.ctrl |= 1 << 2;
    systick.ctrl |= 1 << 0;                 /* 使能Systick */
    systick.load = 0x0fffffff;              /* 注意systick计数器24位，所以这里设置最大重装载值 */
    systick.ctrl |= 1 << 1;                 /* 开启SYSTICK中断 */
    systick.load = 100000;                  /* 每1/delay_ostickspersec秒中断一次 */

}

#[repr(C)]
struct RccTypeDef {
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

fn sys_clock_set(plln:u32, pllm:u32, pllp:u32, pllq:u32) -> u8{
    let rcc = unsafe {&mut *(RCC_BASE as *mut RccTypeDef)};
    let flash = unsafe {&mut *(FLASH_R_BASE as *mut FlashTypeDef)};
    let pwr = unsafe {&mut *(PWR_BASE as *mut PwrTypeDef)};
    let mut retry:u32 = 0;
    let mut retval:u8 = 0;
    let mut swsval:u8 = 0;

    rcc.cr |= 1 << 16; /* HSEON = 1, 开启HSE */

    while ((rcc.cr & (1 << 17)) == 0) && (retry < 0x7FFF) {
        retry += 1;        /* 等待HSE RDY */
    }

    if retry == 0x7FFF {
        retval = 1;     /* HSE无法就绪 */
        return retval;
    }

    rcc.apb1enr |= 1 << 28;                /* 电源接口时钟使能 */
    pwr.cr |= 3 << 14;                     /* 高性能模式,时钟可到168Mhz */
    
    rcc.pllcfgr |= 0x3F & pllm;            /* 设置主PLL预分频系数,  PLLM[5:0]: 2~63 */
    rcc.pllcfgr |= plln << 6;              /* 设置主PLL倍频系数,    PLLN[8:0]: 192~432 */
    rcc.pllcfgr |= ((pllp >> 1) - 1) << 16;/* 设置主PLL的p分频系数, PLLP[1:0]: 0~3, 代表2~8分频 */
    rcc.pllcfgr |= pllq << 24;             /* 设置主PLL的q分频系数, PLLQ[3:0]: 2~15 */
    rcc.pllcfgr |= 1 << 22;                /* 设置主PLL的时钟源来自HSE */

    rcc.cfgr |= 0 << 4;                    /* HPRE[3:0]  = 0, AHB  不分频, rcc_hclk1/2/3 = pll_p_ck */
    rcc.cfgr |= 5 << 10;                   /* PPRE1[2:0] = 5, APB1 4分频   rcc_pclk1 = pll_p_ck / 4 */
    rcc.cfgr |= 4 << 13;                   /* PPRE2[2:0] = 4, APB2 2分频   rcc_pclk2 = pll_p_ck / 2 */

    rcc.cr |= 1 << 24;                     /* 打开主PLL */

    retry = 0;
    while (rcc.cr & (1 << 25)) == 0 {      /* 等待PLL准备好 */
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
    
    rcc.cfgr |= 2 << 0;                    /* 选择主PLL作为系统时钟 */
    
    retry = 0;
    while swsval != 3                     /* 等待成功将系统时钟源切换为pll_p_ck */
    {
        swsval = (rcc.cfgr as u8 & 0x0C) >> 2;   /* 获取SWS[1:0]的状态, 判断是否切换成功 */
        retry += 1;

        if retry > 0x1FFFFF
        {
            retval = 4; /* 无法切换时钟 */
            break;
        }
    }

    return retval;

}

fn clock_init(){
    let rcc = unsafe {&mut *(RCC_BASE as *mut RccTypeDef)};
    rcc.cr = 0x00000001;           /* 设置HISON, 开启内部高速RC振荡，其他位全清零 */
    rcc.cfgr = 0x00000000;         /* CFGR清零 */
    rcc.pllcfgr = 0x00000000;      /* PLLCFGR清零 */
    rcc.cir = 0x00000000;          /* CIR清零 */
    sys_clock_set(336, 8, 2, 7);
}

#[repr(C)]
struct GPIOTypeDef
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

fn sys_gpio_set(p_gpiox: &mut GPIOTypeDef, pinx:u16, mode:u32, otype:u32, ospeed:u32, pupd:u32) {
    let mut pos:u32;
    let mut curpin:u32;
    for pinpos in 0..16 {
        pos = 1 << pinpos;
        curpin = (pinx as u32) & pos;
        if curpin == pos {
            p_gpiox.moder &= !(3 << (pinpos * 2));
            p_gpiox.moder |= mode << (pinpos * 2);
            if (mode == 1) || (mode == 2) {
                p_gpiox.ospeedr &= !(3 << (pinpos * 2));
                p_gpiox.ospeedr |= ospeed << (pinpos * 2);
                p_gpiox.otyper &= !(1 << pinpos);
                p_gpiox.otyper |= otype << pinpos;
            }
            p_gpiox.pupdr &= !(3 << (pinpos * 2));
            p_gpiox.pupdr |= pupd << (pinpos * 2);
        }
    }
}

fn sys_gpio_pin_set(p_gpiox: &mut GPIOTypeDef, pinx:u32, status:bool)
{
    if status {
        p_gpiox.bsrr |= pinx;
    }
    else {
        p_gpiox.bsrr |= pinx << 16;
    }
}

fn sys_gpio_af_set(p_gpiox: &mut GPIOTypeDef, pinx:u32, afx:u32)
{
    let mut pos:u32;
    let mut curpin: u32;
    for pinpos in 0..16 {
        pos = 1 << pinpos;      /* 一个个位检查 */
        curpin = pinx & pos;    /* 检查引脚是否要设置 */

        if curpin == pos{
            p_gpiox.afr[pinpos >> 3] &= !(0x0F << ((pinpos & 0x07) * 4));
            p_gpiox.afr[pinpos >> 3] |= afx << ((pinpos & 0x07) * 4);
        }
    }
}

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

fn usart_init(){
    let rcc = unsafe {&mut *(RCC_BASE as *mut RccTypeDef)};
    let usart1 = unsafe {&mut *(USART1_BASE as *mut UsartTypeDef)};
    let sclk:u32 = 84;
    let baudrate:u32 = 115200;

    /* IO 及 时钟配置 */
    rcc.ahb1enr |= 1 << 0;      /* 使能串口TX脚时钟 */
    rcc.ahb1enr |= 1 << 0;      /* 使能串口RX脚时钟 */
    rcc.apb2enr |= 1 << 4;      /* 使能串口时钟 */
    
    let gpioa_base = unsafe { &mut *(GPIOA_BASE as *mut GPIOTypeDef)};

    sys_gpio_set(gpioa_base, 1 << 9, 2, 0, 1, 1);    /* 串口TX脚 模式设置 */
    sys_gpio_set(gpioa_base, 1 << 10, 2, 0, 1, 1);    /* 串口RX脚 模式设置 */

    sys_gpio_af_set(gpioa_base, 1 << 9, 7);    /* TX脚 复用功能选择, 必须设置正确 */
    sys_gpio_af_set(gpioa_base, 1 << 10, 7);    /* RX脚 复用功能选择, 必须设置正确 */

    let temp:u32 = (sclk * 1000000 + baudrate / 2) / baudrate;              /* 得到USARTDIV@OVER8 = 0, 采用四舍五入计算 */
    /* 波特率设置 */
    usart1.brr = temp;       /* 波特率设置@OVER8 = 0 */
    usart1.cr1 = 0;          /* 清零CR1寄存器 */
    usart1.cr1 |= 0 << 12;   /* 设置M = 0, 选择8位字长 */
    usart1.cr1 |= 0 << 15;   /* 设置OVER8 = 0, 16倍过采样 */
    usart1.cr1 |= 1 << 3;    /* 串口发送使能 */
    /* 使能接收中断 */
    usart1.cr1 |= 1 << 2;    /* 串口接收使能 */
    usart1.cr1 |= 1 << 5;    /* 接收缓冲区非空中断使能 */
    // sys_nvic_init(3, 3, USART_UX_IRQn, 2); /* 组2，最低优先级 */
    
    usart1.cr1 |= 1 << 13;   /* 串口使能 */

}

pub fn putc(ch:char)
{
    let usart1 = unsafe {&mut *(USART1_BASE as *mut UsartTypeDef)};
    while (usart1.sr & 0x40) == 0{}     /* 等待上一个字符发送完成 */
    usart1.dr = ch as u32;
}

impl HardWare {
    pub fn board_init() {
        clock_init();
        systick_init();
        usart_init();
    }
    pub fn stack_init(entry: fn(*mut ()), parameter:*mut (),stack_addr:*mut (),exit: fn())->*mut (){
        let mut stk: *mut () = (stack_addr as usize + core::mem::size_of::<u32>()) as *mut ();
        stk = ((stk as usize) & !7) as *mut ();
        stk = (stk as usize - core::mem::size_of::<StackFrame>()) as *mut ();
    
        let stack_frame = stk as *mut StackFrame;
    

        let stack_frame_ref = unsafe { &mut *stack_frame };
        let stack_frame_ptr = stack_frame as *mut u32;
        let stack_frame_len = core::mem::size_of::<StackFrame>() / core::mem::size_of::<u32>();
        for i in 0..stack_frame_len {
            unsafe {
                *stack_frame_ptr.offset(i as isize) = 0xdeadbeef;
            }
        }
    
        stack_frame_ref.exception_stack_frame.r0 = parameter as u32; // r0 : argument
        stack_frame_ref.exception_stack_frame.r1 = 0; // r1
        stack_frame_ref.exception_stack_frame.r2 = 0; // r2
        stack_frame_ref.exception_stack_frame.r3 = 0; // r3
        stack_frame_ref.exception_stack_frame.r12 = 0; // r12
        stack_frame_ref.exception_stack_frame.lr = exit as u32; // lr
        stack_frame_ref.exception_stack_frame.pc = entry  as u32; // entry point, pc
        stack_frame_ref.exception_stack_frame.psr = 0x01000000; // PSR
 
        stack_frame_ref.flag = 0;
        stk
    }
}
