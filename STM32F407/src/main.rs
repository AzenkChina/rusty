#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::{entry, exception};
use stm32f4::stm32f407;


/// Set up PLL to 168MHz from 16MHz HSI
fn rcc_init(peripherals: &mut stm32f407::Peripherals) {
    let rcc = &peripherals.RCC;
    let flash = &peripherals.FLASH;

    // Reset all peripherals
    rcc.ahb1rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb1rstr.write(|w| unsafe { w.bits(0)});
    rcc.ahb2rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb2rstr.write(|w| unsafe { w.bits(0)});
    rcc.ahb3rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb3rstr.write(|w| unsafe { w.bits(0)});
    rcc.apb1rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb1rstr.write(|w| unsafe { w.bits(0)});
    rcc.apb2rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb2rstr.write(|w| unsafe { w.bits(0)});

    // Ensure HSI is on and stable
    rcc.cr.modify(|_, w| w.hsion().set_bit());
    while rcc.cr.read().hsion().bit_is_clear() {}

    // Set system clock to HSI
    rcc.cfgr.modify(|_, w| w.sw().hsi());
    while !rcc.cfgr.read().sws().is_hsi() {}

    // Clear registers to reset value
    rcc.cr.write(|w| w.hsion().set_bit());
    rcc.cfgr.write(|w| unsafe { w.bits(0) });

    // Configure PLL: 16MHz /8 *168 /2, source HSI
    rcc.pllcfgr.write(|w| unsafe {
        w.pllq().bits(4)
         .pllsrc().hsi()
         .pllp().div2()
         .plln().bits(168)
         .pllm().bits(8)
    });
    // Activate PLL
    rcc.cr.modify(|_, w| w.pllon().set_bit());

    // Set other clock domains: PPRE2 to /2, PPRE1 to /4, HPRE to /1
    rcc.cfgr.modify(|_, w|
        w.ppre2().div2()
         .ppre1().div4()
         .hpre().div1());

    // Flash setup: I$ and D$ enabled, prefetch enabled, 5 wait states (OK for 3.3V at 168MHz)
    flash.acr.write(|w| unsafe {
        w.icen().set_bit()
         .dcen().set_bit()
         .prften().set_bit()
         .latency().bits(5)
    });

    // Swap system clock to PLL
    rcc.cfgr.modify(|_, w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}

    // Set up peripheral clocks
    rcc.ahb1enr.modify(|_, w|
        w.gpioaen().enabled()
         .gpioben().enabled()
    );
}

/// Set up the systick to provide a 100ms timebase
fn systick_init(syst: &mut stm32f407::SYST) {
    syst.set_reload((168_000_000 / 8) / 10);
    syst.clear_current();
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    syst.enable_interrupt();
    syst.enable_counter();
}

/// Set up gpio
fn gpio_init(peripherals: &mut stm32f407::Peripherals) {
    let gpioa = &peripherals.GPIOA;
	
    //LED
    gpioa.moder.modify(|_, w| w.moder6().output());
    gpioa.odr.modify(|_, w| w.odr6().clear_bit());

    gpioa.moder.modify(|_, w| w.moder7().output());
    gpioa.odr.modify(|_, w| w.odr7().clear_bit());
}

/// 
fn led_positive(peripherals: &mut stm32f407::Peripherals) {
    let gpioa = &peripherals.GPIOA;
	
    gpioa.odr.modify(|_, w| w.odr6().clear_bit());
    gpioa.odr.modify(|_, w| w.odr7().set_bit());
}

/// 
fn led_negative(peripherals: &mut stm32f407::Peripherals) {
    let gpioa = &peripherals.GPIOA;
	
    gpioa.odr.modify(|_, w| w.odr6().set_bit());
    gpioa.odr.modify(|_, w| w.odr7().clear_bit());
}

#[entry]
fn main() -> ! {
    let mut peripherals = stm32f407::Peripherals::take().unwrap();
    let mut core = stm32f407::CorePeripherals::take().unwrap();

    rcc_init(&mut peripherals);
	gpio_init(&mut peripherals);
    systick_init(&mut core.SYST);

    loop {
		cortex_m::asm::wfi();
		led_positive(&mut peripherals);
		cortex_m::asm::wfi();
		led_negative(&mut peripherals);
    }
}

#[exception]
fn SysTick() {

}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
