use std::io;

fn main() {

    println!("ERNEST & YOUNG GLOBAL LIMITED Nigeria");
    let mut max_name = String::new();
    let mut max_experience = 0;
    
    let mut name_vec = Vec::new();
    let mut experience_vec = Vec::new();

    
    let mut input1 = String::new();
    println!("How many developers applied:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let dev_num = input1.trim().parse().expect("invalid input");


    for i in 0..dev_num{

        println!("DEVELOPER No.{}", i + 1);

        let mut name = String::new();
        println!("Enter name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Enter years of experience:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read line");
        let experience:i16 = input3.trim().parse().expect("Invalid input");

        // Check if this developer has more experience than the current max
        if experience > max_experience {
            max_name = name;
            max_experience = experience;
        }
        name_vec.push(name);
        experience_vec.push(experience);

    }

    println!("DEVELOPER INFO:\nNAME:{}\nYEARS OF EXPERIENCE:{}",max_name,max_experience)


}
