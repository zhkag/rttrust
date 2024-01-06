#[derive(Copy, Clone)]
pub struct HardWare {
    
}


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


const SCS_BASE: u32 = 0xE000E000;
const SYS_TICK_BASE: u32 = SCS_BASE + 0x0010;

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
struct RCCTypeDef {
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
struct FLASHTypeDef {
    acr:u32,       //< FLASH access control register,   Address offset: 0x00 */
    keyr:u32,      //< FLASH key register,              Address offset: 0x04 */
    optkeyr:u32,   //< FLASH option key register,       Address offset: 0x08 */
    sr:u32,        //< FLASH status register,           Address offset: 0x0C */
    cr:u32,        //< FLASH control register,          Address offset: 0x10 */
    optcr:u32,     //< FLASH option control register ,  Address offset: 0x14 */
    optcr1:u32,    //< FLASH option control register 1, Address offset: 0x18 */
}

#[repr(C)]
struct PWRTypeDef{
    cr:u32,   //< PWR power control register,        Address offset: 0x00 */
    csr:u32,  //< PWR power control/status register, Address offset: 0x04 */
}

const RCC_BASE: u32 = 0x40023800;
const FLASH_R_BASE: u32 = 0x40023C00;
const PWR_BASE: u32 = 0x40007000;

fn sys_clock_set(plln:u32, pllm:u32, pllp:u32, pllq:u32) -> u8{
    let rcc = unsafe {&mut *(RCC_BASE as *mut RCCTypeDef)};
    let flash = unsafe {&mut *(FLASH_R_BASE as *mut FLASHTypeDef)};
    let pwr = unsafe {&mut *(PWR_BASE as *mut PWRTypeDef)};
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
    let rcc = unsafe {&mut *(RCC_BASE as *mut RCCTypeDef)};
    rcc.cr = 0x00000001;           /* 设置HISON, 开启内部高速RC振荡，其他位全清零 */
    rcc.cfgr = 0x00000000;         /* CFGR清零 */
    rcc.pllcfgr = 0x00000000;      /* PLLCFGR清零 */
    rcc.cir = 0x00000000;          /* CIR清零 */
    sys_clock_set(336, 8, 2, 7);
}


impl HardWare {
    pub fn board_init() {
        clock_init();
        systick_init();
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
