use std::io;

fn main() {

    let mut value = loop {
        println!("Enter the number of degrees:");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    break temp;
    };
    
    loop { 
        println!("Is your number in F or C?");
    
        let mut scale = String::new();

        io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line");
        
        let scale: char = match scale.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,     //Error handling isn't great here. If you input a string the program goes back to line 22,
        };                          //but if you enter a char other than F or C the program goes to line 46. But it doesn't break.

        if scale == 'F' {
            value = (value-32.0)*(5.0/9.0);
            println!("The temperature converted to Celcius is: {value}");
            break;
        }
        if scale == 'C' {
            value = value*(9.0/5.0)+32.0;
            println!("The temperature converted to Farenheit is: {value}");
            break;
        }
        else {
            println!("Incorrect value entered.");
        }
    }
}
