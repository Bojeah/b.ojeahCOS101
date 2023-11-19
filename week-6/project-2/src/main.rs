use std::io;
fn main(){

    let mut name = String::new();
    let mut paper_num= String::new();

    println!("RESEARCHERS PUBLICATION INCENTIVE SYSTEM (RPIS)
             by the Nigerian Researchers Guide (NRG)");

    println!("\nplease fill in the form with the required information to detremine your annual incentive");

    for _ in 0..500{

    println!("\nFULLNAME:");
    io::stdin().read_line(&mut name).expect("Not a valid string ");
    let name = name.trim().to_uppercase();

    println!("\nNUMBER OF PAPERS PUBLISHED");
    io::stdin().read_line(&mut paper_num).expect("Not a valid string ");
    let paper_num:i32 = paper_num.trim().parse().expect("Not a valid number");

    if paper_num >= 3 && paper_num <= 5{

        println!("\nPUBLISHER'S INFORMATION");
        println!("\nNAME: {}",name);
        println!("\nBASED OFF THE NUMBER OF PAPERS YOU HAVE PUBLISHED,\nYOUR INCENTIVE IS : N500,000");
    }

    if paper_num > 5 && paper_num < 10{

        println!("\nPUBLISHER'S INFORMATION");
        println!("\nNAME: {}",name);
        println!("\nBASED OFF THE NUMBER OF PAPERS YOU HAVE PUBLISHED,\nYOUR INCENTIVE IS : N800,000");
    }

    if paper_num >= 10 {

        println!("\nPUBLISHER'S INFORMATION");
        println!("\nNAME: {}",name);
        println!("\nBASED OFF THE NUMBER OF PAPERS YOU HAVE PUBLISHED,\nYOUR INCENTIVE IS : N1,000,000");
    }

    if paper_num >= 0 && paper_num < 3{

        println!("\nPUBLISHER'S INFORMATION");
        println!("\nNAME: {}",name);
        println!("\nBASED OFF THE NUMBER OF PAPERS YOU HAVE PUBLISHED,\nYOUR INCENTIVE IS : N100,000");
    }
}}
