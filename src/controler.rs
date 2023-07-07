pub mod trafficlight;
use rp2040_hal as hal;

pub struct Controller
{
   pub lights: [trafficlight::TrafficLight;1],
}
impl Controller
{
   pub fn new(pins: hal::gpio::Pins) -> Self
   {
      
      let a: trafficlight::TrafficLight = 
            trafficlight::TrafficLight::new(pins);
      
      Self{
         lights : [a]
      }
   }

   pub fn logic(&mut self)
   {
      self.lights[0].SetState(trafficlight::State::Red);
   }

}