use std::io;

fn main(){
   loop{

   let mut distance = String::new();
   let mut time = String::new();


   println!("\nWhat is the distance travelled by the car (in miles)?");
   io::stdin().read_line(&mut d).expect("Not a valid string");
   let mut distance:f32 = distance.trim().parse().expect("Not a valid distance");
   distance = distance * 1.609344; // to change from km to miles


   println!("\nHow long did it take for the car to travel that distance (in hours)");
   io::stdin().read_line(&mut t).expect("Not a valid string");
   let time:f32 = time.trim().parse().expect("Not a valid time");

   let speed:f32 = distance / time; //formular for speed

   println!("\nThe speed of the car is {}km/h", speed);

   let mut confirmation = String::new();

   println!("\nDo you want to recalculate speed?. Y / N");

   io::stdin().read_line(&mut confirmation).expect("Not a valid string");
   let confirmation = confirmation.trim();
   
   if confirmation == "N" {
    break
   }







}
}
