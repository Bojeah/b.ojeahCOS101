use std::io;

fn main() {
    println!("MULTIPLICATION TABLE GENERATOR");


    let mut n = String::new();
    
    println!("Enter the value for the last multiplication table you would like to view: ");
    io::stdin().read_line(&mut n).expect("Not a valid string");
    let n:i32 = n.trim().parse().expect("Not a valid integer");

        
    for i in 1..=n{
        for j in 1..=12{
            let result = i * j;
            println!("{} x {} = {}", i, j, result);
        }
        println!("");}}