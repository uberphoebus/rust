fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r1}");

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}