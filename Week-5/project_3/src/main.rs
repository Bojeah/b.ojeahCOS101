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
  let menu_prices:[f32;5] = [3200.0, 3000.0, 2500.0, 2000.0, 2500.0];
  let menu_items:[&str;5] = ["Poundo Yam/Edinkaiko Soup", "Fried Rice & Chicken", "Amala & Ewedu Soup", "Eba & Egusi Soup","White Rice & Stew"];
loop{
 println!("PLEASE ENTER THE ITEM NO. OF WHAT YOU WOULD LIKE TO ORDER");

  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Not valid string");
  let input1:i32 = input1.trim().parse().expect("Not a valid number");
  
  println!("\nHow many portions of {} would you like to order", menu_items[(input1 - 1) as usize]);

  let mut quantity = String::new();
  io::stdin().read_line(&mut quantity).expect("Not a valid string");
  let quantity:f32 = quantity.trim().parse().expect("Not a valid number");
  
  let total_amount = menu_prices[(input1 - 1) as usize] * quantity;

 println!("\nThe total cost of your order is N{}",total_amount);

       if total_amount > 10000.0{
        println!("\nYour order is above N10,000");
        println!("\nHOORAY!! YOU QUALIFY FOR THE DISCOUNT");


        let discounted_amount = total_amount * 0.95;
        println!("\nYour discounted cost is N{}", discounted_amount);
      } 

      println!("\nEnjoy your meal");break;
}
}