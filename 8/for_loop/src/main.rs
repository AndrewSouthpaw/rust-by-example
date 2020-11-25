fn main() {
    println!("Printing 1 through 9");
    for i in 1..10 {
        println!("{}", i);
    }

    println!("Printing 1 through 10");
    for i in 1..=10 {
        println!("{}", i);
    }

    let nums = [2, 4, 6, 8];

    for n in nums.iter() {
        // borrow syntax is used to ensure we aren't changing
        // the array or values
        if n == &8 {
            println!("{}", n);
        } else {
            print!("{}, ", n);
        }
    }

    println!("Who do we appreciate?!");
}
