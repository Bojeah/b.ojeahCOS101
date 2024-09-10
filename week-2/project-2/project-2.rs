fn main() {
    // item amounts would be represented by the first letter of the item name
    let t:f64 = 450_000.00;
    let m:f64 = 1_500_000.00;
    let h:f64 = 750_000.00;
    let d:f64 = 2_850_000.00;
    let a:f64 = 250_000.00;

    // quantity of each item would be "q+(first letter of the item name)"
    let qt:f64 = 2.0;
    let qm:f64 = 1.0;
    let qh:f64 = 3.0;
    let qd:f64 = 3.0;
    let qa:f64 = 1.0;
    
    // total number of items(sum of item quantity) would be represented with q
    let q = qt + qm + qh + qd + qa;
    
    // sum would be represented by s
    let s = (t * qt) + (m * qm) + (h * qh) + (d * qd) + (a * qa);
    println!("The total amount made from the sales of the items in the sales record by P.M. Okeke and Sons Ltd is {:.2}",s );
    
    //the average would be represented by x
    let x = s / q;
    println!("The average amount made from the sales of the items in the sales record by P.M. Okeke and Sons Ltd is {:.2}",x );






}