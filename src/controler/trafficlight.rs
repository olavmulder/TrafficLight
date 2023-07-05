#![allow(non_snake_case)]


use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
use rp2040_hal;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;
use bsp::hal::{
   pac,
   sio::Sio,
};


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
   pub fn new() -> Self
   {
      let mut pac = pac::Peripherals::take().unwrap();
      let sio = Sio::new(pac.SIO);
      let pins = bsp::Pins::new(
         pac.IO_BANK0,
         pac.PADS_BANK0,
         sio.gpio_bank0,
         &mut pac.RESETS,
      );
      GpioLed { 
         red   : pins.gpio21.into_push_pull_output(),
         green : pins.gpio20.into_push_pull_output(),
         yellow: pins.gpio19.into_push_pull_output(),
      }
   }

   pub fn SetLed(&mut self, state: &State)
   {
      self.red.set_low().unwrap();
      self.green.set_low().unwrap();
      self.yellow.set_low().unwrap();
      match state {
         State::Red=> self.red.set_high().unwrap(),
         State::Green =>self.green.set_high().unwrap(),
         State::Yellow =>self.yellow.set_high().unwrap(), 
      };
   }

}

pub struct TrafficLight
{
   pub state: State,
   led : GpioLed
}


impl TrafficLight
{
   pub fn  new() -> Self
   {
      TrafficLight
      {
         state: State::Red,
         led : GpioLed::new()
      }
   }

   pub fn SetState(&mut self, st: State)
   {
      self.state = st;
      GpioLed::SetLed(&mut self.led, &self.state);
   }
}
