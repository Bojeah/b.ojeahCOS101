fn main() {
    
    // using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of the vector
    println!("\nThe length of the of the Vec::new is: {}",v.len());

    //using macro 
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    //printing size of the vector
    println!("The length of vec macro is: {}",v.len());
}