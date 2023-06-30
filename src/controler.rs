pub mod trafficlight;
use crate::controler::trafficlight::State;

pub struct Controller
{
   pub lights: Vec<trafficlight::TrafficLight>
}

impl Controller
{
   pub fn new() -> Self
   {
      Controller {
         lights: Vec::new()
      }
   }
   pub fn remove(&mut self, id:usize)
   {
      self.lights.remove(id);
   }
   pub fn add(&mut self)
   {
      self.lights.push(trafficlight::TrafficLight::new());
   }

   pub fn change(&mut self, id: usize, state: State)
   {
      if id < self.lights.len() 
      {
         self.lights[id].SetState(state);
      }
   }
   pub fn print(&self)
   {
      for i in 0..self.lights.len()
      {
         println!("light i:{} = {:#?}", i, self.lights[i].state);
      }
   }

}