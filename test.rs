
//like to have Function cammel case
#![allow(non_snake_case)]



fn main()
{
   DataTypes();
   looping(32);
   println!("return conditional: {}", Conditional());
   let mut interger:u32 = 21;
   let name = String::from("Olav");
   FunctionByRef(name, &mut interger);

   TupleFunc();
   ArrayFunc(&mut [1]); 
   let a = vec![1,2,3];
   let b = a;
   let b_ret = Borrowing(b);
   //println!("after borrow: {:?}", b); //not allowed borring 
   println!("after borrow: {:?}", b_ret); // allowed borring 
   
   let mut c:Vec<i32> =  vec![1,2,3];
   Borrowing_Best(&mut c);
   println!("after borrow_best: {:?}", c); // allowed borring 
}

fn 

fn Borrowing_Best(a: &mut Vec<i32>)
{
   a[0] = 3;
}
fn Borrowing(a: Vec<i32>) -> Vec<i32>
{
   println!("{:?}", a); 
   return a;
}
fn TupleFunc()
{
   let name:(u8, i8) = (1, -1);
   println!("{:?}", name.0);
}
fn ArrayFunc(arr_par: &mut [i8;1])
{
   let arr:[u8; 255] = [0; 255];
   println!("{:?}", arr);
   arr_par[0] = 127;
   println!("{:?}", arr_par);
}
fn DataTypes()
{
   //not mutable
   let amount_lights: u8 = 2;
   println!("Hello rust with {} lights", amount_lights);
   //mutable
   let mut changeble_lights:u8 = 3;
   changeble_lights += 1;
   println!("Hello rust with {} lights", changeble_lights);

   //char 
   let mut character_changable  = "b";
   character_changable = "c";
   println!("Hello rust with changed char {}", character_changable);

   //float
   let _float = 1.0;
   println!("Hello rust with changed char {}", _float);

   //constant variable ->data type is required
   const PI:f32 = 3.1415;
   println!("Hello rust with const float pi {}", PI);

   //override variable also type
   let _amount  = 0;
   let _amount = 'a'; 
   println!("Hello rust with overwirte amount: {}", _amount);

   //string literal
   let name: &'static str = "Olav";

   //mutable string
   let string_variable = String::new();
   println!("length is {}",string_variable.len());

   let string_variable = String::from("Olav");
   println!("length is {}",string_variable.len());
}

fn looping(amount:u32) -> i8
{
   for x in (0..amount).step_by(2)
   {
      println!("{}", x);
   }
   return 0;
}
fn Conditional() -> i8
{
   let num = 12;
   match num 
   {
      12 => {
         return 0;
      }
      _ => {
         return -1;
      }
   }
}

fn FunctionByRef(par_name:String, par:&mut u32)
{
   println!("reference is {}:{}", par_name, *par);
}