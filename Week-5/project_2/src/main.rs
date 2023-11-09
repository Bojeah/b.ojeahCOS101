use std::io;
fn main(){
    
      
       
         println!("\n            EMLPOYEE INCENTIVE HUB\n");

         println!("\nWELCOME TO THE EMPLOYEE INCENTIVE HUB.\nPLEASE ANSWER THE FOLLOWING QUESTIONS TO VIEW YOUR ANNUAL INCENTIVE\n");
    
       loop{
        println!("\nWHAT'S YOUR EMPLOYEE STATUS? (EXPERIENCED / INEXPERIENCED)");
        let mut e = String::new();
    io::stdin().read_line(&mut e).expect("Not a valid response");
    let e = e.trim().to_lowercase();

    if e != "experienced" && e != "inexperienced" {println!("Error!! invalid input. Try again"); continue;}

    println!("\nPlease enter your age ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Not a valid strinng");
    let a:i32 = a.trim().parse().expect("Not a valid age");

    if e == "experienced"{
     if a >= 40 && a <= 65 { //Age of retirement is 65
        println!("\n Your Incentive is  ₦18,720,000 per Annum");break;}
        
    
    else if a >= 30 && a < 40{
       println!("\n Your Incentive is  ₦17,760,000 per Annum");break;
       
    }
     else if a <= 17 && a>= 1{
      println!("\n ERROR! Age not within legal working class. Try again");continue;}

      else if a <= 200 && a>= 66{
      println!("\n ERROR! Age not within legal working class. Try again");continue;
   }
       else if a <= 1000000 && a>=201 {
      println!("\n ERROR! Age not not realistic. Try again");continue;}

       else if a <= 0 && a>= -1000000 {
      println!("\n ERROR! Age not not realistic. Try again");continue;
   }
    else if a < 30 && a >= 18{ //legal minimum age for work is 18 
       println!("\n Your Incentive is ₦12,600,000 per Annum");break;
       }
    }
    
    else if e == "inexperienced"{
      if a >= 18 && a <= 65{
       println!("\nYour Incentive is  ₦1,200,000 per Annum");break;
      }

        else if a <= 17 && a>= 1{
      println!("\n ERROR! Age not within legal working class");continue;
    }
       else if a >=  66 && a <= 200 {
         println!("\n ERROR! Age not within legal working class");continue;

       } 
       else if a <= 1000000 && a>=201 {
      println!("\n ERROR! Age not not realistic");continue;}

       else if a <= 0 && a>= -1000000 {
      println!("\n ERROR! Age not not realistic");continue;
   }
       else if a <= 0 && a>= 201{
      println!("\n ERROR! Age not not realistic");continue;
   }
    }
    
 }}