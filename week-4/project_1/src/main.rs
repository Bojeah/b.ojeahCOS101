use std::io;

fn main(){
   loop{

   let mut d = String::new();
   let mut t = String::new();


   println!("\nWhat is the distance travelled by the car (in miles)?");
   io::stdin().read_line(&mut d).expect("Not a valid string");
   let mut d:f32 = d.trim().parse().expect("Not a valid distance");
   d = d * 1.609344;


   println!("\nHow long did it take for the car to travel that distance (in hours)");
   io::stdin().read_line(&mut t).expect("Not a valid string");
   let t:f32 = t.trim().parse().expect("Not a valid time");

   let s:f32 = d / t;

   println!("\nThe speed of the car is {}km/h", s);

   let mut e = String::new();
   println!("\nDo you want to continue. Y / N");
   io::stdin().read_line(&mut e).expect("Not a valid string");
   let e = e.trim();
   if e == "N" {
    break
   }







}
}
