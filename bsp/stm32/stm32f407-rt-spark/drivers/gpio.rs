const PERIPH_BASE: u32 = 0x40000000;
const AHB1PERIPH_BASE: u32 = PERIPH_BASE + 0x00020000;
const RCC_BASE: u32 = AHB1PERIPH_BASE + 0x3800;
const AHB1ENR: u32 = RCC_BASE + 0x30;
const GPIOF_BASE: u32 = AHB1PERIPH_BASE + 0x1400;

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