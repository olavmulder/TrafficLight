//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod controler;

use panic_halt as _;
// Alias for our HAL crate
use rp2040_hal as hal;
use rp2040_hal::Clock;
use rp2040_boot2;
// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;


#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;



#[rp2040_hal::entry]
fn main() -> !
{
   let mut pac = pac::Peripherals::take().unwrap();
   let core = pac::CorePeripherals::take().unwrap();
   let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

   // Configure the clocks
   let clocks = hal::clocks::init_clocks_and_plls(
      XTAL_FREQ_HZ,
      pac.XOSC,
      pac.CLOCKS,
      pac.PLL_SYS,
      pac.PLL_USB,
      &mut pac.RESETS,
      &mut watchdog,
   ).ok().unwrap();
   // The single-cycle I/O block controls our GPIO pins
   let sio = hal::Sio::new(pac.SIO);

   // Set the pins to their default state
   let pins = hal::gpio::Pins::new(
      pac.IO_BANK0,
      pac.PADS_BANK0,
      sio.gpio_bank0,
      &mut pac.RESETS,
   );
   
   let mut c1 = controler::Controller::new(pins);
   let mut delay = cortex_m::delay::Delay::new(core.SYST,clocks.system_clock.freq().to_Hz());
   
   unsafe {
      pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
   }
   loop
   {
      c1.logic();
      //cortex_m::asm::wfi();
      delay.delay_ms(1000);
   }
}