use std::io;
fn main(){

    println!("PAU 2023 STUDENT COUNCIL ELECTION PORTAL");
    println!("please input your details below");
    for x in 1..=150{
    loop{
    let mut stu_name = String::new();
    let mut stu_mail = String::new();
    let mut department = String::new();
    let mut stu_origin = String::new();
    let mut stu_level = String::new();
    let mut stu_cgpa = String::new();
    let mut class_rep = String::new();

    println!("\nFullname:");
    io::stdin().read_line(&mut stu_name).expect("Not a valid string");
    let stu_name = stu_name.trim().to_uppercase();
    

    println!("\nEmail:");
    io::stdin().read_line(&mut stu_mail).expect("Not a valid string");
    let stu_mail = stu_mail.trim();


    println!("\nDepartment (In full):");
    io::stdin().read_line(&mut department).expect("Not a valid string");
    let department = department.trim().to_lowercase();

    // all the courses offered in pau
    // an error will be flagged if the input isnt a course offered in pau
    if department != "mechanical engineering" &&
       department != "information science and media studies" &&
       department != "computer science" &&
       department != "business administration" &&
       department != "economics" &&
       department != "mass communication" &&
       department != "software engineering" &&
       department != "financial accounting" &&
       department != "electrical/electronics engineering"{
        println!("ERROR!! Not a course in this university. please check your input and try again");
        break;
       }

    println!("\nLevel:");
    io::stdin().read_line(&mut stu_level).expect("Not a valid string");
    let stu_level = stu_level.trim();

    // mechanical and ee engineering are the only courses with 500 level
    if department == "mechanical engineering" || department == "electrical/electronics engineering"{
        if stu_level != "100" && stu_level != "200" && stu_level != "300" && stu_level != "400" && stu_level != "500"{
        println!("ERROR!! not a level in this department. Please check your input and try again");
        break;
    }
}
        
    else {
         if stu_level != "100" && stu_level != "200" && stu_level != "300" && stu_level != "400"{
        println!("ERROR!! not a level in this department");
        break;
     }
 }
        
    println!("\nCGPA (in 2 d.p):");
    io::stdin().read_line(&mut stu_cgpa).expect("Not a valid string");
    let stu_cgpa:f32 = stu_cgpa.trim().parse().expect("CGPA not valid");
    
    // cgpa can technically not be less than zero although it is impossible to get less than 1  
    if stu_cgpa < 0.0 && stu_cgpa > 5.0{
        println!("ERROR!! CGPA invalid. please check your input and try again")
    }


    println!("\nState of origin:");
    io::stdin().read_line(&mut stu_origin).expect("Not a valid string");
    let stu_origin = stu_origin.trim().to_uppercase();

    println!("\nAre you currenly a Class Rep? yes/no ");
    io::stdin().read_line(&mut class_rep).expect("Not a valid");
    let class_rep = class_rep.trim().to_lowercase();

     if class_rep != "yes" && class_rep != "no"{
        println!("invalid input");
     }

     if class_rep == "yes" && stu_level != "100" && stu_cgpa > 4.0{
        println!("\nCANDIDATE INFORMATION
            \nNAME: {}
            \nEMAIL: {}
            \nDEPARTMENT:{}
            \nSTATE OF ORIGIN: {}
            \nCANDIDATE NO.: {}",stu_name, stu_mail, department,stu_origin,x);
        
        println!("\nYOU CAN VOTE");
    
        break;
     }

   else{
    println!("\nSORRY, YOU ARE NOT ELIGIBLE TO VOTE");
    continue;

}
}
}
}