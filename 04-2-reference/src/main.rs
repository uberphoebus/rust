fn main() {
    // ownership
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    // reference
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1); // pass reference -> borrow
    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    let mut s = String::from("hello");
    change(&mut s);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}