pub mod trafficlight;
use crate::controler::trafficlight::State;

pub struct Controller
{
   pub lights: [trafficlight::TrafficLight;1],
   //delay: cortex_m::delay::Delay
}

impl Controller
{
   pub fn new() -> Self
   {
      //let mut pac = pac::Peripherals::take().unwrap();
      //let core = pac::CorePeripherals::take().unwrap();
      //let watchdog = Watchdog::new(pac.WATCHDOG);
      // External high-speed crystal on the pico board is 12Mhz
      //let external_xtal_freq_hz = 12_000_000u32;
      /*let clocks = init_clocks_and_plls(
         external_xtal_freq_hz,
         pac.XOSC,
         pac.CLOCKS,
         pac.PLL_SYS,
         pac.PLL_USB,
         &mut pac.RESETS,
         &mut watchdog,
      ).ok().unwrap();*/
      let a: trafficlight::TrafficLight = trafficlight::TrafficLight::new();
      //let b: trafficlight::TrafficLight = trafficlight::TrafficLight::new();
      //let c: trafficlight::TrafficLight = trafficlight::TrafficLight::new();
      Self{
         //delay  : cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz()),
         lights : [a]
      }
   }

   pub fn change(&mut self, id: usize, state: State)
   {
      if id < self.lights.len() 
      {
         self.lights[id].SetState(state);
      }
   }
   pub fn logic(&mut self)
   {
      /*self.lights[0].SetState(State::Red);
      self.delay.delay_ms(5000);
      self.lights[0].SetState(State::Green);
      self.delay.delay_ms(5000);
      self.lights[0].SetState(State::Yellow);
      self.delay.delay_ms(1000);*/
   }
   /*pub fn print(&self)
   {
      for i in 0..self.lights.len()
      {
         println!("light i:{} = {:#?}", i, self.lights[i].state);
      }
   }*/

}