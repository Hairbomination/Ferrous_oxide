use std::collections::HashMap;

fn main()
{
    let mut v = vec![17, 2, 4, 6, 19, 23, 29, 23, 17, 4, 9, 17, 2, 6, 25, 11, 23, 8, 10, 22, 4, 8, 23, 12, 5, 23, 14, 17, 9, 2, 1, 16];

    print!("The list of numbers is: ");
    for element in &v {
        print!("{element} ");     //The print! macro omits the newline. println! was printing each element on a new line which sucked.
    }
    print!("\n");
    
    let vsort = &mut v;     //Because; ownership. Ugh
    vsort.sort();

    print!("The sorted list is: ");
    for element in &v { 
        print!("{element} ");
    }
    
    println!("\nThe median is: {}", v[v.len()/2]);

    let mut map = HashMap::new();

    for element in &v {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    
    let mut max_val = 0;           //Pinched off StackOverflow, which I'm not proud of. But I had a good idea of what I wanted it to do
    let mut max_key = 0;           //and this code matches what I was planning so I guess it's not too bad. S.O. says there are library
    for (k, v) in map.iter() {     //functions which do the same thing but I don't understand any of them so I didn't use them.
        if *v > max_val {
            max_val = *v;
            max_key = **k;         //Flaw: I haven't handled the case where two or more keys contain the same value. 
        }
    }
    println!("The number that occurs most often is: {max_key}");
}
