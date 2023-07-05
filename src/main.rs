//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod controler;

use rp_pico as bsp;
use bsp::entry;
use defmt_rtt as _;

#[entry]
unsafe fn main() -> !
{
   
   let mut c1 = controler::Controller::new();

   const RED:controler::trafficlight::State = controler::trafficlight::State::Red;
   const GREEN:controler::trafficlight::State = controler::trafficlight::State::Green;
   const YELLOW:controler::trafficlight::State = controler::trafficlight::State::Yellow;
   loop
   {
      c1.change(0, GREEN);
      c1.change(0, YELLOW);
      c1.change(0, RED);
   }
}