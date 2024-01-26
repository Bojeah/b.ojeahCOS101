use std::io::Read;
use::std::io;

fn main(){
    
    println!("Please enter: \n(A) If you are an administrator \n(P) If you are a project manager 
(S) If you are a staff member \n(C) If you are a customer \n(V) If you are a vendor");
   
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("failed to read");
    let response = response.trim().to_lowercase();

    let input = ["a", "p", "e", "c", "v"];
    let user = [administrator_table, projectmanager_table, employee_table, customer_table, vendor_table];

    for x in 0..5 {
     
     if response == input[x]{

        user[x]();
        return;
     }

    }

    println!("invalid input");
     

}

fn administrator_table(){

    let mut file_1 = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents_1 = String::new();
    file_1.read_to_string(&mut contents_1).unwrap();
    print!("{}", contents_1);

}

fn projectmanager_table(){

    let mut file_2 = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents_2 = String::new();
    file_2.read_to_string(&mut contents_2).unwrap();
    print!("{}", contents_2);
}

fn employee_table() {

    let mut file_3 = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents_3 = String::new();
    file_3.read_to_string(&mut contents_3).unwrap();
    print!("{}", contents_3);
}

fn customer_table() {
    
    let mut file_4 = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents_4 = String::new();
    file_4.read_to_string(&mut contents_4).unwrap();
    print!("{}", contents_4);
}

fn vendor_table() {

    let mut file_5 = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents_5 = String::new();
    file_5.read_to_string(&mut contents_5).unwrap();
    print!("{}", contents_5);
}
