use std::io;

struct Laptop {
    quantity:u32,
    price:u32,
}

impl Laptop{
    fn cost(&self)->u32{
        self.quantity * self.price
    }
}


fn main(){
    //making somewhat of a menu for the user to see what and how many products are in stock

    println!("ITEMS LIST\nITEM(LAPTOP) IN STOCK\tPRICE(₦)\nHP\t\t10\t650,000
IBM\t\t06\t755,000
TOSHIBA\t\t10\t550,000
DELL\t\t04\t850,000");

//vector to store the costs of the different items
    let mut costs = Vec::new();

    println!("\nHow many HP laptops do you want");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read");
    let hp_num:u32 = input1.trim().parse().expect("Invalid input");
    
//use if statement to make sure user's order is within what is available    
    if hp_num > 10 {
        println!("NO. of laptops needed is more than No. of laptops in stock");
        return;
    }
    else{
        let hp = Laptop {
            quantity:hp_num,
            price:650000
        };
        costs.push(hp.cost());
    }

    println!("\nHow many IBM laptops do you want");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read");
    let ibm_num:u32 = input2.trim().parse().expect("Invalid input");

    if ibm_num > 6 {
        println!("NO. of laptops needed is more than No. of laptops in stock");
        return;
    }
    else{
        let ibm = Laptop {
            quantity:ibm_num,
            price:755000
    };
    costs.push(ibm.cost());
}


    println!("\nHow many Toshiba laptops do you want");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read");
    let toshiba_num:u32 = input3.trim().parse().expect("Invalid input");

    if toshiba_num > 10 {
        println!("NO. of laptops needed is more than No. of laptops in stock");
        return;
    }
    else{
        let toshiba = Laptop {
            quantity:toshiba_num,
            price:850000
    };
    costs.push(toshiba.cost());
}


    println!("\nHow many Dell laptops do you want");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Failed to read");
    let dell_num:u32 = input4.trim().parse().expect("Invalid input");

    if dell_num > 4 {
        println!("NO. of laptops needed is more than No. of laptops in stock");
        return;
    }
    else{
        let dell = Laptop {
            quantity:dell_num,
            price:550000
    };
    costs.push(dell.cost());
}

     let mut total_cost = 0;

//to calculate the total cost of all products inquired for

     for k in 0..4{
        total_cost += costs [k];
     }
  
//a reciept for the summary of the purchase      
      println!("\nRECEIPT\nITEM\t    QUANTITY\tUNIT PRICE(₦)\tAMOUNT(₦)
HP     \t\t{}\t 650000\t\t{}
IBM    \t\t{}\t 755000\t\t{}
Toshiba\t\t{}\t 850000\t\t{}
Dell   \t\t{}\t 550000\t\t{}
\nTOTAL: ₦{}",hp_num,costs[0],ibm_num,costs[1],
toshiba_num,costs[2],dell_num,costs[3],total_cost); 
}