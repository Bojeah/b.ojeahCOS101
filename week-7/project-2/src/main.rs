use std::io;

fn main(){
    let mut input1 = String::new();
    println!("How many siblings do you have");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let sib_num = input1.trim().parse().expect("Invalid input");

    let mut info_name: Vec<String> = Vec::new();
    let mut info_age:Vec<u16> = Vec::new();
    let mut info_marr_sta:Vec<String> = Vec::new();
    let mut info_sib_sta = Vec::new();
    let mut info_sib_sch1 = Vec::new();
    let mut info_sib_course = Vec::new();
    let mut info_sib_kids = Vec::new();
    let mut info_sib_reside = Vec::new();
    let mut info_sib_waec = Vec::new();
    let mut info_sib_sch2 = Vec::new();
    let mut info_sib_level = Vec::new();
    
    for x in 1..=sib_num{
    
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();
    let mut input12 = String::new();

        println!("\nSibling no.{}",x);

         println!("\nEnter sibling name");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let sib_name = input2.trim().to_string();
        
        println!("How old is {}",sib_name);
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let sib_age:u16 = input3.trim().parse().expect("Invalid input");

        if sib_age > 18{
            println!("what is {}'s marital status (single/married)",sib_name);
            io::stdin().read_line(&mut input4).expect("Failed to read input");
            let marr_sta = input4.trim().to_uppercase();
            

            if  marr_sta == "SINGLE"{
                println!("Is {} a worker or student(worker/student)", sib_name);
                io::stdin().read_line(&mut input5).expect("Failed to read input");
                let sib_sta = input5.trim().to_uppercase();
               

                if sib_sta == "STUDENT"{
                    println!("What University does {} attend",sib_name);
                    io::stdin().read_line(&mut input6).expect("Failed to read input");
                    let sib_sch1 = input6.trim().to_uppercase();
                    info_sib_sch1.push(sib_sch1);

                    println!("What course is {} studying",sib_name);
                    io::stdin().read_line(&mut input7).expect("Failed to read input");
                    let sib_course = input7.trim().to_uppercase();
                    info_sib_course.push(sib_course);
                
                 info_sib_sta.push(sib_sta);
             }

             else if sib_sta == "WORKER"{
                let sib_sch1 = "N/A".to_string();
                let sib_course = "N/A".to_string();

                info_sib_sch1.push(sib_sch1);
                info_sib_course.push(sib_course);
                info_sib_sta.push(sib_sta);
             }
              let sib_kids = 0;
              let sib_reside = "N/A".to_string();

              info_sib_kids.push(sib_kids);
              info_sib_reside.push(sib_reside);
         }             

                else if marr_sta == "MARRIED"{
                    println!("How many children does {} have", sib_name);
                    io::stdin().read_line(&mut input8).expect("Failed to read input");
                    let sib_kids:u16 = input8.trim().parse().expect("Invalid input");
                    info_sib_kids.push(sib_kids);

                    println!("What city does {}'s family live in", sib_name);
                    io::stdin().read_line(&mut input9).expect("Failed to read input");
                    let sib_reside = input9.trim().to_uppercase();
                    info_sib_reside.push(sib_reside);
            
                    let sib_sta = "N/a".to_string(); 
                    info_sib_sta.push(sib_sta); 

                    let sib_sch1 = "N/A".to_string();
                    let sib_course = "N/A".to_string(); 

                    info_sib_course.push(sib_course);
                    info_sib_sch1.push(sib_sch1);
             
            }
            let sib_waec = "N/a".to_string();
            let sib_sch2 = "N/a".to_string();
            let sib_level = "N/a".to_string();
           
            
            info_sib_waec.push(sib_waec);
            info_sib_level.push(sib_level);
            info_sib_sch2.push(sib_sch2);
            info_marr_sta.push(marr_sta);
        }
            
        

        else if sib_age <= 18{
            println!("Has {} written the waec examination (yes/no)", sib_name);
            io::stdin().read_line(&mut input10).expect("Failed to read input");
            let sib_waec = input10.trim().to_lowercase();

            if sib_waec == "yes"{
                println!("What secondary school did {} attend", sib_name);
                io::stdin().read_line(&mut input11).expect("Failed to read input");
                let sib_sch2 = input11.trim().to_uppercase();
                info_sib_sch2.push(sib_sch2);

                let sib_level = "N/A".to_string();
                info_sib_level.push(sib_level)
            }
            
            else if sib_waec == "no"{
                println!("what is {}'s current level in school", sib_name);
                io::stdin().read_line(&mut input12).expect("Failed to read input");
                let sib_level = input12.trim().to_uppercase();
                info_sib_level.push(sib_level);

                let sib_sch2 = "N/A".to_string();
                 info_sib_sch2.push(sib_sch2);

            }

            else{
                println!("Invalid input");
                break;
            }
            
            info_sib_waec.push(sib_waec);
            let marr_sta = "N/A".to_string();
            let sib_sta = "N/A".to_string();
            let sib_sch1 = "N/A".to_string();
            let sib_course = "N/A".to_string();
            let sib_kids = 0;
            let sib_reside = "N/A".to_string();
            
            info_marr_sta.push(marr_sta);
            info_sib_sch1.push(sib_sch1);
            info_sib_course.push(sib_course);
            info_sib_sta.push(sib_sta);
            info_sib_kids.push(sib_kids);
            info_sib_reside.push(sib_reside);
        }
        
        info_name.push(sib_name);
        info_age.push(sib_age);
}
      
        for n in 0..sib_num{
            println!("\nSIBLING {} INFORMATION", n+1);
            println!("Name : {}", info_name[n] );
            println!("Age : {}", info_age[n] );
            println!("Marital status: {}", info_marr_sta[n]);
            println!("Occupation: {}", info_sib_sta[n]);
            println!("School of study: {}", info_sib_sch1[n]);
            println!("Course of study: {}", info_sib_course[n]);
            println!("No. of kids: {}", info_sib_kids[n]);
            println!("City of residence: {}" ,info_sib_reside[n]);
            println!("Waec status: {}", info_sib_waec[n]);
            println!("Name of school: {}", info_sib_sch2[n]);
            println!("Class grade: {}", info_sib_level[n]);
        }}