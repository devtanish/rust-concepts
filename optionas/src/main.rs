/*
    this is a function that finds the first occurrence of the letter 'a' in a string
    and returns an Option<i32> indicating the index or None
    if 'a' is not found so this function return None
    if 'a' is found, it returns Some(index)

    This Option<i32> returns two possible values:
    1. Some(index): If 'a' is found, it returns the index.
    2. None: If 'a' is not found, it returns None.

    just like creating a enum of 
    enum Option<T> {
        None,
        Some(T),
    }

*/

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let my_string = String::from("raman");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}