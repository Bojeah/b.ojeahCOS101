use std::io::Write;

fn main(){

let student_name = ["STUDENT NAME","\nOluchi Mordi","\nAdams Aliyu","\nShania Bolade","\nAdekunle Gold","\nBlanca Edomoh"];

let matric_no = ["\t| MATRIC. NUMBER","\t| ACC10211111","\t| ECO10110101","\t| CSC10328828","\t| EEE11020202","\t| MEE10202001"];

let dept = ["| DEPARTMENT","\t| Accounting","\t| Economics","\t| Computer","\t| Electrical","\t| Mechanical"];

let level = ["\t| LEVEL","\t| 300","\t| 100","\t| 200","\t| 200","\t| 100"];

let mut file = std::fs::File::create("project.txt").expect("create failed");
file.write_all("\t\t\tPAU SIMS\n".as_bytes()).expect("failed to write");
file.write_all("\t\t\t--------\n\n".as_bytes()).expect("failed to write");

for i in 0..6{
file.write_all(student_name[i].as_bytes()).expect("failed to write");

file.write_all(matric_no[i].as_bytes()).expect("failed to write");

file.write_all(dept[i].as_bytes()).expect("failed to write");

file.write_all(level[i].as_bytes()).expect("failed to write");
}

println!("le finis");
}
