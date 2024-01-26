use std::io;

// Define a struct for the Laptop
struct Laptop {
    quantity: u32,
    price: u32,
}

impl Laptop {
    // Define a method to calculate the cost of the laptops
    fn cost(&self) -> u32 {
        self.quantity * self.price
    }
}

fn main() {
    // Display the items and their details
    println!("ITEMS LIST\n
ITEM(LAPTOP)  | IN STOCK |  PRICE(₦)
--------------+----------+----------
HP            |       10 |  650,000
IBM           |        6 |  755,000
TOSHIBA       |       10 |  850,000
DELL          |        4 |  550,000");

    // Create vectors for laptops, quantities, and prices
    let laptops = vec!["HP", "IBM", "TOSHIBA", "DELL"];
    let instock = vec![10, 6, 10, 4];
    let prices = vec![650000, 755000, 850000, 550000];

    // Vector to store the costs of the different items
    let mut costs = Vec::new();

    // Iterate over the laptops, quantities, and prices simultaneously
    for ((&laptop, &quantity), &price) in laptops.iter().zip(instock.iter()).zip(prices.iter()) {
        println!("\nHow many {} laptops do you want?", laptop);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let quantity_num: u32 = input.trim().parse().expect("Invalid input");

        // Use if statement to make sure the user's order is within what is available
        if quantity_num > quantity {
            println!("NO. of laptops needed is more than No. of laptops in stock");
            return;
        } else {
            // Create a Laptop instance for the current laptop
            let current_laptop = Laptop {
                quantity: quantity_num,
                price,
            };
            costs.push(current_laptop.cost());
        }
    }

    // Calculate the total cost of all products inquired for
    let total_cost: u32 = costs.iter().sum();

    // Display a receipt for the summary of the purchase
    println!("\nRECEIPT\nITEM\t    QUANTITY\tUNIT PRICE(₦)\tAMOUNT(₦)");
    for (((&laptop, &quantity), &price),&cost) in laptops.iter().zip(instock.iter()).zip(prices.iter()).zip(costs.iter()) {
        println!("{:<8}\t{}\t{:<14}\t{}", laptop, quantity, price, cost);
    }
    println!("\nTOTAL: ₦{}", total_cost);
}
