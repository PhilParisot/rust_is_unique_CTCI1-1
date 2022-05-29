use std::collections::HashSet;

fn main() {
    let string = "abcdefg";
    println!("{}", string);

    hash_set_is_unique(string);
    in_place_no_sort_is_unique(string);
    sort_is_unique(string);
}

fn hash_set_is_unique(string: &str) {
    let mut unique_chars: HashSet<char> = HashSet::new();
    for c in string.chars() {
        if !unique_chars.contains(&c) {
            unique_chars.insert(c);
        } else {
            println!("String is not unique");
            return;
        }
    }
    println!("String is unique");
}

fn in_place_no_sort_is_unique(string: &str) {
    for (i, x) in string.chars().enumerate() {
        for c in string[i + 1..].chars() {
            if x == c {
                println!("String is not unique");
                return;
            }
        }
    }
    println!("String is unique");
}

fn sort_is_unique(string: &str) {
    let mut sorted_string: Vec<char> = string.chars().collect();
    sorted_string.sort();
    for (i, c) in sorted_string[1..].iter().enumerate() {
        if &sorted_string[i] == c {
            println!("String is not unique");
            return;
        }
    }
    println!("String is unique");
}
