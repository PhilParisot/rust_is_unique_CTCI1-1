use std::collections::HashSet;

fn main() {
    let mut uniqueChars: HashSet<char> = HashSet::new();
    let string = "abcdefg";
    println!("{}", string);

    for c in string.chars() {
        if !uniqueChars.contains(&c) {
            uniqueChars.insert(c);
        } else {
            println!("String is not unique");
            return;
        }
    }
    println!("String is unique");
}
