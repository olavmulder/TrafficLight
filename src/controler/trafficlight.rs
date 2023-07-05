#![allow(non_snake_case)]


use defmt_rtt as _;
use embedded_hal::digital::v2::{ToggleableOutputPin};
use panic_probe as _;
use rp2040_hal;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;

#[derive(PartialEq)] pub enum State {
   Red = 0,
   Yellow ,
   Green,
}

use crate::controler::trafficlight::rp2040_hal::gpio::bank0::Gpio21;
use crate::controler::trafficlight::rp2040_hal::gpio::bank0::Gpio20;
use crate::controler::trafficlight::rp2040_hal::gpio::bank0::Gpio19;
use crate::controler::trafficlight::rp2040_hal::gpio::PushPullOutput;

struct GpioLed
{
   red   :  rp2040_hal::gpio::Pin<Gpio21,PushPullOutput>,
   green :  rp2040_hal::gpio::Pin<Gpio20,PushPullOutput>,
   yellow:  rp2040_hal::gpio::Pin<Gpio19,PushPullOutput>,
}

impl GpioLed
{
   pub fn new(pins: bsp::Pins) -> Self
   {
      
      GpioLed { 
         red   : pins.gpio21.into_push_pull_output(),
         green : pins.gpio20.into_push_pull_output(),
         yellow: pins.gpio19.into_push_pull_output(),
      }
   }

   pub fn ToggleRedLed(&mut self)
   {
      self.red.toggle().unwrap();
   }

}

pub struct TrafficLight
{
   pub state: State,
   led : GpioLed
}


impl TrafficLight
{
   pub fn  new(pins: bsp::Pins) -> Self
   {
      TrafficLight
      {
         state: State::Red,
         led : GpioLed::new(pins)
      }
   }

   pub fn SetState(&mut self, st: State)
   {
      self.state = st;
      GpioLed::ToggleRedLed(&mut self.led);
   }
}
