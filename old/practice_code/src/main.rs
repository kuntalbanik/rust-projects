// Recursion function
// call itself
fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
    } else {
        println!("{seconds} seconds to blastoff!");
        countdown(seconds - 1);
    }
}

fn main() {
    //   let result = mystery();
    let number = 10;

    // Match statement with multiple values and condition
    match number {
        value if value % 2 == 0 => println!("{value} is even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }

    /*


    */
    // Call recursion function
    countdown(number);
}
