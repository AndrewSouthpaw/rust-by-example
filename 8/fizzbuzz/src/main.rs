fn main() {
    let mut n = 0;

    loop {
        n += 1;

        if n > 15 {
            break;
        } else if n == 9 {
            println!("Skipping 9");
            continue; // skip 9
        } else if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz {}", n);
        } else if n % 3 == 0 {
            println!("Fizz {}", n);
        } else if n % 5 == 0 {
            println!("Buzz {}", n);
        }
    }
}
