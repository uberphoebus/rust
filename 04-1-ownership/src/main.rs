fn main() {

    // scope of variable
    {
        let s = "hello";
        println!("{}", s);
    } // s is out of scope

    // memory allocation
    {
        let s = String::from("hello");
        println!("{}", s);
    } // s is out of scope -> memory is deallocated


    // move
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}", s1); // error: value borrowed here after move
        println!("{}", s2); // move ownership to s2
    }

    // clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // copy
    {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }

    // ownership and functions
    {
        let s = String::from("hello");
        takes_ownership(s);
        // println!("{}", s); // error: value borrowed here after move

        let x = 5;
        makes_copy(x);
        println!("{}", x); // i32 is Copy -> x is still valid
    }

    // return values and scope
    {
        let s1 = gives_ownership(); // ownership is moved to s1
        println!("{}", s1);

        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2); // ownership is moved to s3
        // println!("{}", s2); // error: value borrowed here after move
        println!("{}", s3);
    }
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("makes copy : {}", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

