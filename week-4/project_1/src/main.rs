use std::io;

fn main(){

   let mut d = String::new();
   let mut t = String::new();


   println!("Enter your distance (in miles)");
   io::stdin().read_line(&mut d).expect("Not a valid string");
   let mut d:f32 = d.trim().parse().expect("Not a valid distance");
   d = d * 1.609344;


   println!("Enter you time (in hours)");
   io::stdin().read_line(&mut t).expect("Not a valid string");
   let t:f32 = t.trim().parse().expect("Not a valid time");

   let s:f32 = d / t;

   println!("The speed of the f1 car is {}km/h", s);







}
