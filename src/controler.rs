pub mod trafficlight;
use crate::controler::trafficlight::State;

use rp_pico as bsp;

pub struct Controller
{
   pub lights: [trafficlight::TrafficLight;1],
}
impl Controller
{
   pub fn new(pins: bsp::Pins) -> Self
   {
      
      let a: trafficlight::TrafficLight = 
            trafficlight::TrafficLight::new(pins);
      Self{
         lights : [a]
      }
   }

   pub fn logic(&mut self)
   {
      self.lights[0].SetState(State::Red);
   }

}