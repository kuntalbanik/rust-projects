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
    let number = 10;

    /*


    */
    // Call recursion function
    countdown(number);
}
