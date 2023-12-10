use std::io::Write;

fn main(){

    let larger = ["LARGER","\n33 Export", "\nDesperados","\nGoldberg","\nGulder","\nHeineken","\nStar"];

    let stout = ["\t\t| STOUT","\t| Legend","\t| Turbo king","\t| Williams","\t\t|","\t|","\t\t|"];

    let non_alchoholic = ["\t\t| NON-ALCHOHOLIC","\t| Maltina","\t| Amstel Malta","\t| Malta Gold", "\t\t| Fayrouz","\t\t|","\t\t|"];
    
    let mut file = std::fs::File::create("drinks.txt").expect("Failed to create");
    
    file.write_all("\t\tNIGERIAN BREWERY LIMITED\n".as_bytes()).expect("Failed to write");
    file.write_all("\t\t------------------------\n\n".as_bytes()).expect("Failed to write");
    for x in 0..7{
    file.write_all(larger[x].as_bytes()).expect("Failed to write");

    file.write_all(stout[x].as_bytes()).expect("Failed to write"); 

    file.write_all(non_alchoholic[x].as_bytes()).expect("Failed to write");   
}

    println!("\nDone");    
}

