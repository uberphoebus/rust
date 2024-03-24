enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    
    let mut v1 = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];
    println!("third = {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("third = {third}"),
        None => println!("no third"),
    }

    let v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0];
    // v3.push(6);
    println!("first = {first}");

    // loop
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}