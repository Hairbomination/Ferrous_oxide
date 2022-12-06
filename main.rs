use std::io;

fn main() {

    println!("I generate the nth number in the Fibonacci sequence.");
    
    let value: u64 = loop {

        println!("Enter a number for n:");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        if n == 0 {
            println!("There's no such thing as zeroth. Enter a positive integer for n");
        }

        else {
        break n;
        };
    };

    if value == 1 {
        println!("The first Fibonacci number is 0");
    }
    else {
        let mut counter = value;
        let mut a: u64 = 0;     //Some of these explicit types are probably unnecessary 
        let mut b: u64 = 1;     //but I cbf deleting and testing to find out.
        let mut c: u64 = 1;

        while counter > 2 {
            c = a + b;
            a = b;
            b = c;
            counter -= 1;
        };
    println!("The Fibonacci number in position {value} is: {c}");
    }
}