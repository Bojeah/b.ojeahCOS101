fn main() {
	// the first letter of each parameter is used to represent each of them
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;
    // a is the amount after 5 years 
	let a = p * (1.00 + (r / 100.00)).powf(n);
	println!("The Amount the Ibeju local Government Chairman would have to pay to sterling bank after 5 years is {:.2}", a);
	
    // ci is the compound interest after 5 years
	let ci = a - p;
	println!("The compound interest sterling bank would get from the Ibeju local Government Chairman after 5 year is {:.2}", ci);