fn main() {
    println!("On the first day of Christmas my true love sent to me a partridge in a pear tree");

    let mut d_index = 1;
    let mut g_index = 1;
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", 
    "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["and a partridge in a pear tree.",
        "two turtle doves,",
        "three french hens,",
        "four calling birds,",
        "five gold rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming"];

    while d_index < 12 {
        println!("On the {} day of Christmas my true love sent to me {}", days[d_index], gifts[g_index]);
        while g_index != 0 {
            g_index -= 1;
            println!("{}", gifts[g_index]);
        }
        d_index += 1; 
        g_index = d_index;   
    }
}
