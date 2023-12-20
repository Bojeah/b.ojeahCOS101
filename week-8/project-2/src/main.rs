use std::io;

fn main() {

    println!("ERNEST & YOUNG GLOBAL LIMITED Nigeria");
    let mut max_experience = 0;
    let mut max_name = "";
    let mut max_age = 0;
    
    let mut name_vec:Vec<String> = Vec::new();
    let mut age_vec:Vec<u16> = Vec::new();
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

        println!("Enter age:");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read line");
        let age:u16 = input4.trim().parse().expect("Invalid input");


        println!("Enter years of experience:");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read line");
        let experience:u16 = input3.trim().parse().expect("Invalid input");

         name_vec.push(name);
        experience_vec.push(experience);
        age_vec.push(age);
    }

        // Check if this developer has more experience than the current max
        for x in 0..dev_num{
        if experience_vec[x] > max_experience {
            max_name = &name_vec[x];
            max_experience = experience_vec[x];
            max_age = age_vec[x];
        }}

    println!("DEVELOPER INFO:\nNAME: {}AGE: {}\nYEARS OF EXPERIENCE: {}",max_name,max_age,max_experience)


}
