#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32l4;
extern crate panic_halt;
#[cfg(debug_assertions)]
extern crate cortex_m_semihosting;

use cortex_m_rt::{entry, exception};
use stm32l4::stm32l4x6;
#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;


#[link(name = "greet", kind = "static")]
extern {
	fn greet() -> u32;
}


/// Set up RCC
fn rcc_init(peripherals: &mut stm32l4x6::Peripherals) {
    let rcc = &peripherals.RCC;
	
    // Reset all peripherals
    rcc.ahb1rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb1rstr.write(|w| unsafe { w.bits(0)});
    rcc.ahb2rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb2rstr.write(|w| unsafe { w.bits(0)});
    rcc.ahb3rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb3rstr.write(|w| unsafe { w.bits(0)});
    rcc.apb1rstr1.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb1rstr1.write(|w| unsafe { w.bits(0)});
    rcc.apb1rstr2.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb1rstr2.write(|w| unsafe { w.bits(0)});
    rcc.apb2rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb2rstr.write(|w| unsafe { w.bits(0)});

    // Set up peripheral clocks
    rcc.ahb2enr.modify(|_, w|
        w.gpioaen().set_bit()
    );
}

/// Set up the systick to provide a 100ms timebase
fn systick_init(syst: &mut stm32l4x6::SYST) {
    syst.set_reload((16_000_000 / 8) / 10);
    syst.clear_current();
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    syst.enable_interrupt();
    syst.enable_counter();
}

/// Set up gpio
fn gpio_init(peripherals: &mut stm32l4x6::Peripherals) {
    let gpioa = &peripherals.GPIOA;

    //LED
    gpioa.moder.modify(|_, w| w.moder5().output());
    gpioa.odr.modify(|_, w| w.odr5().clear_bit());
}

///
fn led_positive(peripherals: &mut stm32l4x6::Peripherals) {
    let gpioa = &peripherals.GPIOA;

    gpioa.odr.modify(|_, w| w.odr5().set_bit());
}

///
fn led_negative(peripherals: &mut stm32l4x6::Peripherals) {
    let gpioa = &peripherals.GPIOA;

    gpioa.odr.modify(|_, w| w.odr5().clear_bit());
}

#[entry]
fn main() -> ! {
    let mut peripherals = stm32l4x6::Peripherals::take().unwrap();
    let mut core = stm32l4x6::CorePeripherals::take().unwrap();

    rcc_init(&mut peripherals);
	gpio_init(&mut peripherals);
    systick_init(&mut core.SYST);

#[cfg(debug_assertions)]
	hprintln!("Call C function, value is {}.", unsafe {greet()}).unwrap();

#[cfg(debug_assertions)]
	hprintln!("Call C function, value is {}.", unsafe {greet()}).unwrap();

#[cfg(debug_assertions)]
	hprintln!("Hello, World!").unwrap();

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
