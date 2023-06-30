#![allow(non_snake_case)]
#[derive(Debug)]
#[derive(PartialEq)] pub enum State {
   Red = 0,
   Yellow ,
   Green,
}

pub struct TrafficLight
{
   pub state: State,
}


impl TrafficLight
{
   pub fn  new() -> Self
   {
      TrafficLight
      {
         state: State::Red,
      }
   }

   pub fn SetState(&mut self, st: State)
   {
      self.state = st;
   }


   pub fn GetState(&self) -> &State
   {
      return &self.state;
   }
}
