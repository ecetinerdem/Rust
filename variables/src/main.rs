use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let z = five();
    println!("The value of z is: {z}");

    let m = plus_one(5);
    println!("The value of m is: {m}");
}

fn another_function(x: i32) {
    println!("The value of x: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(m: i32) -> i32 {
    m + 1
}
