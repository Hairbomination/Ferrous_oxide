use std::collections::HashMap;
use std::io;

fn main() {

    let mut company = HashMap::new();

    company.insert(String::from("Corporate"), vec![String::from("Abby"), String::from("Bob"), String::from("Carlos")]);
    company.insert(String::from("HR"), vec![String::from("Donna"), String::from("Edith")]);
    company.insert(String::from("IT"), vec![String::from("Running"), String::from("Out"), String::from("Of")]);
    company.insert(String::from("Customer Service"), vec![String::from("Ideas"), String::from("Why"), String::from("Does")]);
    company.insert(String::from("Sales"), vec![String::from("This"), String::from("Need"), String::from("So"), String::from("Much")]);
    company.insert(String::from("Finance"), vec![String::from("Typing"), String::from("EnoughAlready")]);

    loop {
        let option = loop {
            println!("What would you like to do today?
    
    Select 1 to add a new employee
    Select 2 to see a list of all employees in a department
    Select 3 to see a list of all people in the company sorted by department
    Select 4 to exit");
    
            let mut option = String::new();

            io::stdin()
                .read_line(&mut option)
                .expect("Failed to read line");
        
            let option: i8 = match option.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        
        if option > 0 && option < 5 {
            break option;
        }
        else {continue;}
        };
        match option {
            1 => add_employee(&mut company),
            2 => print_employees(&mut company),
            3 => print_company(&mut company),
            4 => break,
            _ => print!("Run. Fast"),
        }
    }
}

fn add_employee(add_emp: &mut HashMap<String, Vec<String>>) {  //Adds an employee to the chosen department
    println!("What is the employees name?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let department = loop {                 //Lets the user choose the department by number
        println!("What Department is {name} joining?
            1 = Corporate
            2 = HR
            3 = Customer Service
            4 = IT
            5 = Sales
            6 = Finance:");
        
        let mut department = String::new();
            
        io::stdin()
            .read_line(&mut department)
            .expect("Failed to read line");

        let department: i8 = match department.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if department > 0 && department < 7 {
            break department;
        }
        else {continue;}
        }; 
        
    let mut choice = String::new();
    match department {
        1 => choice = String::from("Corporate"),     //Need to convert the number to a word matching the department names
        2 => choice = String::from("HR"),
        3 => choice = String::from("Customer Service"),
        4 => choice = String::from("IT"),
        5 => choice = String::from("Sales"),
        6 => choice = String::from("Finance"),
        _ => println!("Oh the humanity!"),
    }
    
    let ugh = &name;         //This is all weird and has something to do with the ownership rules
    for (k, v) in add_emp {  //This adds the name to the end of the vector values in the correct department 
        if *k == choice {
            v.push(ugh.to_string());
        }
    }
    println!("Welcome to {choice}, {name}");
}

fn print_employees(print_emp: &mut HashMap<String, Vec<String>>) {     //This prints a list of employees in a department
    println!("Which department?");

    let department = loop {
        println!("
            1 = Corporate
            2 = HR
            3 = Customer Service
            4 = IT
            5 = Sales
            6 = Finance:");
        
        let mut department = String::new();
            
        io::stdin()
            .read_line(&mut department)
            .expect("Failed to read line");

        let department: i8 = match department.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if department > 0 && department < 7 {
            break department;
        }
        else {continue;}
        };
        
        match department {    //Converts the number to a word matching the department name
            1 => {let choice = String::from("Corporate"); print_dept(&choice, print_emp)},
            2 => {let choice = String::from("HR"); print_dept(&choice, print_emp)},
            3 => {let choice = String::from("Customer Service"); print_dept(&choice, print_emp)},
            4 => {let choice = String::from("IT"); print_dept(&choice, print_emp)},
            5 => {let choice = String::from("Sales"); print_dept(&choice, print_emp)},
            6 => {let choice = String::from("Finance"); print_dept(&choice, print_emp)},
            _ => println!("Oh the humanity!"),
        }
}

fn print_company(print_co: &mut HashMap<String, Vec<String>>) {     //Prints the employee list sorted by department
    println!("This is the full list of employees:");

    sort_company(print_co);

    for (key, value) in print_co {
        print!("{key}: {:?} ", value);
    }
    print!("\n\n");
}

fn sort_company(sort_co: &mut HashMap<String, Vec<String>>) -> &HashMap<String, Vec<String>> {
                                                                      //This sorts the employee names in each department alphabetically
    for (_k, v) in sort_co.iter_mut() {                               //Departments are not sorted alphabetically
        v.sort();
    }
    
    return sort_co;
}

fn print_dept(dept: &str, print_emp: &mut HashMap<String, Vec<String>>) {   //This prints all of the employees in a department sorted alphabetically
   sort_company(print_emp);

   println!("The employees in {dept} are: ");

   for (k, v) in print_emp {        //I got this bit of code right the first time. No errors
    if *k == *dept {
        for i in v {
            print!("{i} ");
        }
        println!("\n\n");
    }
   } 
}
