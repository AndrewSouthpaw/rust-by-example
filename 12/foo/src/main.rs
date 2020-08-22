use rand::Rng;

fn main() {
    println!("Hello, world!");
    let number = rand::thread_rng().gen_range(1, 11);
    println!("The number is: {}", number); 
}
