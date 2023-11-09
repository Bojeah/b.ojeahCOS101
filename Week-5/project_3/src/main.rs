use std::io;

fn main() {
  println!("WELCOME TO BEN'S EATERY!!
                                   
                                    MENU

     item No.      ITEMS                        PRICE(per portion)
        1      Poundo Yam/Edinkaiko Soup           - N3,200
        2      Fried Rice & Chicken                - N3,000
        3      Amala & Ewedu Soup                  - N2,500
        4      Eba & Egusi Soup                    - N2,000
        5      White Rice & Stew                   - N2,500


            ATTENTION!!! CUSTOMERS WITH ORDERS ABOVE N1000 GET 5% OFF THEIR ORDER!!!
");
loop{
let p:f32 = 3200.0;
let f:f32 = 3000.0;
let a:f32 = 2500.0;
let e:f32 = 2000.0;
let w:f32 = 2500.0;


//this  is  very unnecessary but i just think it makes the code more personalized
let pp = "Poundo Yam/Edinkaiko Soup";
let ff = "Fried Rice & Chicken";
let aa = "Amala & Ewedu Soup";
let ee = "Eba & Egusi Soup";
let ww = "White Rice & Stew";
  println!("PLEASE ENTER THE ITEM NO. OF WHAT YOU WOULD LIKE TO ORDER");
  let mut t = String::new();
  io::stdin().read_line(&mut t).expect("Not valid string");
  let t = t.trim();

  if t == "1"{

   println!("\nHow many portions of {} would you like to order", pp);
    }

    else if t == "2"{
   println!("\nHow many portions of {} would you like to order", ff);
    }

   else if t == "3"{
   println!("\nHow many portions of {} would you like to order", aa);
    }

    else if t == "4"{
   println!("\nHow many portions of {} would you like to order", ee);
    }

    else if t == "5"{
   println!("\nHow many portions of {} would you like to order", ww);
    } 

    else {println!("\nERROR!! Not an item number on the menu. Try again");continue;
    }


  let mut q = String::new();
  io::stdin().read_line(&mut q).expect("Not a valid string");
  let q:f32 = q.trim().parse().expect("Not a valid number ");
  
  let mut c = 0.0;

  if t == "1"{let j = p * q;
    println!("\nThe total cost of your order is N{}",j);
    c += j;
    }

    else if t == "2"{ let j = f * q;
    println!("\nThe total cost of your order is N{}",j);
         c += j;
    }

    else if t == "3"{ let j = a * q;
    println!("\nThe total cost of your order is N{}",j);
         c += j;
    }

    else if t == "4"{ let j = e * q;
    println!("\nThe total cost of your order is N{}",j);
     c += j;
    }

    else if t == "5"{let j = w * q;
    println!("\nThe total cost of your order is N{}",j);
       c += j;
    }
      

       if c > 10000.0{
        println!("\nYour order is above N10,000");
        println!("\nHOORAY!! YOU QUALIFY FOR THE DISCOUNT");
        let f = c - (c * 0.05);
        println!("\nYour discounted cost is N{}", f);
      } 

      println!("\nEnjoy your meal");break;
}
}