use std::io;
fn main(){
    
      
       
         println!("\n            EMLPOYEE INCENTIVE HUB\n");

         println!("\nWELCOME TO THE EMPLOYEE INCENTIVE HUB.\nPLEASE ANSWER THE FOLLOWING QUESTIONS TO DETERMINE YOUR ANNUAL INCENTIVE\n");
    
       loop{
        println!("\nHOW MANY YEARS OF WORK EXPERIENCE DO YOU HAVE");

    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Not a valid response");
    let experience:i32 = experience.trim().parse().expect("Not a valid number");

    if experience > 47 {//65 - 18. the minimum and maximum ages for working
    println!("ERROR!! Years of experience not possible, check your input and try again.");continue;}

    println!("\nPlease enter your age ");

    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Not a valid strinng");
    let age:i32 = age.trim().parse().expect("Not a valid age");

     let z:i32 = age - experience;
    
    if experience > age {
      println!("ERROR!! Years of experience Cannot be more than age, check your input and try again." );continue;
    }
    if  z < 18{      // if z is less than 18 that would mean that the employee started working before 18
      println!("ERROR!! impossible to have that much years of experience at such age, check your inputs and try again");continue;
    }
    if experience >= 10{
     if age >= 40 && age <= 65 { //Age of retirement is 65
       println!("\n Your Incentive is  ₦18,720,000 per Annum");break;
      }  
    
    else if age >= 30 && age < 40{
       println!("\n Your Incentive is  ₦17,760,000 per Annum");break;
       
    }
     else if age <= 17 && age >= 1{
      println!("\n ERROR! Age not within legal working class. Try again");continue;}

      else if age <= 200 && age >= 66{
      println!("\n ERROR! Age not within legal working class. Try again");continue;
   }
       else if age <= 1000000 && age >=201 {
      println!("\n ERROR! Age not not realistic. Try again");continue;}

       else if age <= 0 && age>= -1000000 {
      println!("\n ERROR! Age not not realistic. Try again");continue;
   }
    else if age < 30 && age >= 18{ //legal minimum age for work is 18 
       println!("\n Your Incentive is ₦12,600,000 per Annum");break;
       }
       else if age < 28 && age >= 18 {
        println!("ERROR!! Not possible to have ten or more years of work experience at this age, check age and try again");break;
       }
    }
    
    else if experience < 10{
      if age >= 18 && age <= 65{
       println!("\nYour Incentive is  ₦1,200,000 per Annum");break;
      }

        else if age <= 17 && age >= 1{
      println!("\n ERROR! Age not within legal working class");continue;
    }
       else if age >=  66 && age <= 200 {
         println!("\n ERROR! Age not within legal working class");continue;

       } 
       else if age <= 1000000 && age >=201 {
      println!("\n ERROR! Age not not realistic");continue;}

       else if age <= 0 && age >= -1000000 {
      println!("\n ERROR! Age not not realistic");continue;
   }
       else if age <= 0 && age >= 201{
      println!("\n ERROR! Age not not realistic");continue;
   }
    }
    
 }}