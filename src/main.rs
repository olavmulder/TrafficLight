//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod controler;
use bsp::entry;
use rp_pico as bsp;
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
unsafe fn main() -> !
{
   let mut pac = pac::Peripherals::take().unwrap();
   let core = pac::CorePeripherals::take().unwrap();
   let mut watchdog = Watchdog::new(pac.WATCHDOG);
   let sio = Sio::new(pac.SIO);

   // External high-speed crystal on the pico board is 12Mhz
   let external_xtal_freq_hz = 12_000_000u32;
   let clocks = init_clocks_and_plls(
      external_xtal_freq_hz,
      pac.XOSC,
      pac.CLOCKS,
      pac.PLL_SYS,
      pac.PLL_USB,
      &mut pac.RESETS,
      &mut watchdog,
   ).ok().unwrap();
   let pins = bsp::Pins::new(
      pac.IO_BANK0,
      pac.PADS_BANK0,
      sio.gpio_bank0,
      &mut pac.RESETS,
   );
   let mut c1 = controler::Controller::new(pins);
   let mut delay = cortex_m::delay::Delay::new(core.SYST,clocks.system_clock.freq().to_Hz());
   
   loop
   {
      c1.logic();
      delay.delay_ms(1000);
   }
}