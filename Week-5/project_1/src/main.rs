//program for calculating the roots of a quadratic equation

use std::io;
fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
     
    println!("\n                        QUADRATIC EQUATION SOLVER\n");
    
    println!("YOUR EQUATION SHOULD BE IN THE FORM: ax^2 + bx + c = 0\n");

    println!("\nEnter the value for a "); //coefficient of x^2
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("\nEnter the value for b"); //coefficient of x
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("\nEnter the value for c"); 
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");


    let discriminant:f32 = ((b).powf(2.0)) + (-1.0 * 4.0 * a * c); //discriminant

    let root1:f32 = ((-1.0 * b) + (discriminant.sqrt())) / (2.0 * a) ;

    let root2:f32 = ((-1.0 * b) - (discriminant.sqrt())) / (2.0 * a) ;   


    if discriminant < 0.0{
        println!("\nThere are no real roots for this equation.");
    }
     
     else if discriminant == 0.0{
       println!("\nThe equation has two real, identical/the same roots.");
    
        println!("\nThe roots of the equation are {} and {}",root1, root2); 
  
    }

     else if discriminant > 0.0 {
        println!("\nThe equation has two real, distinct roots.");

        println!("\nThe roots of the equation are {} and {}",root1, root2);   
    
     }

    



    
}