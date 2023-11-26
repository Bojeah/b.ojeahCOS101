use std::io;

fn trapezium(){

    println!("AREA OF TRAPEZIUM");

   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("\nInput trapezium's height (in cm)");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let height:f32 = input1.trim().parse().expect("Invalid input");
   
   println!("Input length of the trapezium's first base (in cm)");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let base1:f32 = input2.trim().parse().expect("Invalid input");

   println!("Input length of the trapezium's second base (in cm)");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let base2:f32 = input3.trim().parse().expect("Invalid input");

   let trap_area:f32 = (0.5 * height) * (base1 + base2);

   println!("The area of the trapezium is {:.2}cm²",trap_area);
}


fn rhombus(){

    println!("AREA OF RHOMBUS");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nInput length of the first diagonal (in cm)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let dia1:f32 = input1.trim().parse().expect("Invalid input");

    println!("Input length of the second diagonal (in cm)");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let dia2:f32 = input2.trim().parse().expect("Invalid input");
    
    let rhombus_area:f32 = 0.5 * dia1 * dia2;

    println!("The area of the rhombus is {:.2}cm²",rhombus_area );
}


fn parallelogram(){
    println!("\nAREA OF PARALLELOGRAM");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nInput length of the parallelogram's base (in cm)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base:f32 = input1.trim().parse().expect("Invalid input");

    println!("Input the parallelogram's altitude(in cm)");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let alt:f32 = input2.trim().parse().expect("Invalid input");

    let para_area:f32 = base * alt;
    
    println!("The area of the parallelogram is {:.2}cm²",para_area);
}

fn cube(){

    println!("\nVOLUME OF  CUBE");

    let mut input = String::new();

    println!("\nInput the length of sides of the cube");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let length:f32 = input.trim().parse().expect("Invalid input");

    let cube_vol:f32 = 6.0 * (length.powf(2.0));

    println!("The volume of the cube is {:.2}cm³", cube_vol);
}

fn cylinder(){

    println!("\nVOLUME OF  CYLINDER");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nInput value for radius of the base (in cm)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius:f32 = input1.trim().parse().expect("Invalid input");

    println!("Input value for height (in cm)");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height:f32 = input2.trim().parse().expect("Invalid input");

    let cylin_vol:f32 = (3.142) * (radius.powf(2.0)) * height;

    println!("The volume of the cylinder is {:.2}cm³",cylin_vol );
}


fn main(){
    println!("AREA / VOLUME CALCULATOR");
    println!("
              \n1. area of a trapezium\n2. area of a rhombus
3. area of a parallelogram\n4. volume a of cube
5. volume of a cylinder");
    println!("Please input the number attached to what you would like to calculate");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input:i32 = input.trim().parse().expect("Invalid input");

    let arr = [trapezium,rhombus,parallelogram,cube,cylinder];
    
    if input >= 1 && input <= 5{
        arr[(input - 1)as usize]();
    }
    else{
        println!("Invalid input");
    }
}