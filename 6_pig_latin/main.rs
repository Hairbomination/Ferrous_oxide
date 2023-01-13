use std::io;

fn main() {
    println!("What text would you like me to convert to pig latin?");

    let mut sample = String::new();

    io::stdin()
        .read_line(&mut sample)
        .expect("Failed to read line");    //I tried to do some exception handling here (reject any text containing numbers),
                                           //but it got too complicated so I gave up
    println!("You entered: {sample}");

    let mut vec: Vec<String> = Vec::new();

    for word in sample.split_whitespace() {    //Takes the text and splits it into words.
    vec.push((*word).to_string());            //Words are added to a vector
    }

    for i in vec.iter_mut() {
        
        let mut firstletter = (i.chars().next().unwrap()).to_string();  //This finds the first character of the string
        if is_vowel(&firstletter) == false {                            //Had to make the variable a string with to_string because I don't know
            firstletter.push_str("ay");                                 //any way to add a char to a string
            i.push_str("-");     //This is annoying. I couldn't figure out how to combine lines 25 and 26 into one expression.
            i.push_str(&firstletter);
            i.remove(0);                 //I think this removes the first byte from the string. Might error if text contains chars larger than 1 byte.
        }
        else {
            i.push_str("-hay");
        }
        print!("{i} ");
    }
    
    fn is_vowel(x: &String) -> bool {
        let ch: char = x.chars().next().unwrap();
        if ch == 'a' || ch == 'A' || ch == 'e' || ch == 'E' || ch == 'i' ||    //ffs I need a switch statement but I don't think Rust has them
            ch == 'I' || ch == 'o' || ch == 'O' || ch == 'u' || ch == 'U' {    //and I haven't read the chapter about Match statements yet.
            return true;
        }
        else { return false; }
    }
}
