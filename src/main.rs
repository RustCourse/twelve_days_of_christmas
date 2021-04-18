const DAYS: [&str; 12] = [
    "first", "second", "third", "forth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const WORDS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    let mut words_n = 1;
    for day in DAYS.iter() {
        println!("On the {} day of Christmas, my true love sent to me", *day);
        for line in (0..words_n).rev() {
            println!("{}", WORDS[line]);
        }
        words_n += 1;
        println!();
    }
}
