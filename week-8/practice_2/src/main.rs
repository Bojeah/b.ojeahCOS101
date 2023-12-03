fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];

    let mut input1 = String::new();

    println!("Enter an index value btw (0-7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    //index is a non negative value that is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("Invalid input");

    //getting value at given index value 
    let ch: char = v[index];

    print!("{} is a character for index [{}]\n", ch, index);

}