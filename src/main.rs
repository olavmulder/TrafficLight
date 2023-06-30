mod controler;

fn main()
{
   let mut c1 = controler::Controller::new();
   c1.add();
   c1.add();
   //println!("len is {}", c1.lights.len());

   c1.change(0, controler::trafficlight::State::Green);
   //println!("state is {:#?}", c1.lights[0].GetState());
   c1.print();
   logic(&mut c1);
}


fn logic(c1: &mut controler::Controller)
{
   const RED:controler::trafficlight::State = controler::trafficlight::State::Red;
   const GREEN:controler::trafficlight::State = controler::trafficlight::State::Green;
   const YELLOW:controler::trafficlight::State = controler::trafficlight::State::Yellow;

   loop
   {
      c1.change(0, RED);
      c1.change(1, GREEN);
      //sleep (5)
      c1.change(1, YELLOW);
      //sleep (1)
      c1.change(1, RED);
      //sleep(1)
      c1.change(0, GREEN);
      //sleep(5)
      c1.change(0, YELLOW);
      //sleep(1)
   }
}