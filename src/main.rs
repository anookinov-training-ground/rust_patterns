fn main() {
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }    

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = 5; // let PATTERN = EXPRESSION;
    println!("{}", x);

    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);

    // let (x, y) = (1, 2, 3); // compiler error - use _ or .. to ignore

    let point = (3, 5);
    print_coordinates(&point);

    // let Some(x) = some_option_value; // compiler error from attempt to use a refutable pattern with let

    // fix refutable pattern where an irrefutable pattern is needed with if let instead of let
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }

    // warning because it doesn't make sense to use if let with an irrefutable pattern
    // if let x = 5 {
    //     println!("{}", x);
    // }

    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn foo(x: i32) {
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
