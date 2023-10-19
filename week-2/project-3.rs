fn main() {
	// the first letter of each parameter would be used to represent it
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
    
	let a = p * (1.0 - (r / 100.0)).powf(n);
	println!("The value of Mrs. Grace's Tv after 3 years would be {:.2}", a);
	}