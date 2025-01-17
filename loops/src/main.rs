fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println! {"The result is {result}"}

    lableloop();
}

fn lableloop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}")
}

fn whloop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!");
}

/// This function will print out the values of the array `a` in order.
/// It does this by looping through the array with a `while` loop and
/// printing out the value at each index.
fn whileloop() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

    /// This function prints out the elements of the array `a`
    /// in order. It does this by looping through the array with
    /// a `for` loop and printing out each element.
fn forloop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

/// This function prints numbers from 3 to 1 in reverse order, followed by "LIFTOFF!!".
/// It utilizes a reverse iterator on the range 1 to 4 (exclusive).

fn reverse_forloop() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}
