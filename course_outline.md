
## 1. Hello World

Comments: `//` or `/* */`
Print with `println!`
Interpolate: `println!("{} days a year, {} hours a day", 365, 24);`
Interpolate named: `println!("Hello, {name}", name="Andrew");`

Debugging with `{:?}`, pretty printing with `{:#?}`
To debug structs, you must derive from `fmt::Debug`

```rust
#[derive(Debug)]
struct Structure(i32);

println!("Now {:?} will print!", Structure(3));
```

## 2. Primitives

**Scalar types**

1. Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (pointer)
1. Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (pointer)
1. Floating: `f32`, `f64`
1. `char` unicode scalar values (4B ea)
1. `bool`
1. Unit type `()`

**Compound types**

1. arrays `[1, 2, 3]` (always homogeneous)
1. tuples `(1, true)` (heterogeneous)

Printing arrays:

```rust
let my_array = [1, 2, 3];
// println!("This will not work: {}", my_array)
println!("This will work: {:?}", my_array);
```

Destructuring tuples:

```rust
//tuples can be destructured to create bindings
let tuple = (1, "hello", 4.5, true);

let (a, b, c, d) = tuple
println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}
```

Arrays have a type and length: `[T; size]`
Slices have a type and indefinite length: `&[T]`

`&` is for "borrowing"

```rust
// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
}
```

## 4. Variables

Variables receive type annotations implicitly by default

Variable bindings are immutable by default:

```rust
let mut mutable_binding = 1;
mutable_binding += 1;

let immutable_binding = 5;
immutable_binding += 1; // nope.
```

Variables live inside a *block* defined by `{}`.

```rust
fn main() {
  let long_lived_binding = 1;

  {
    let short_lived_binding = 2;
    let long_lived_binding = 5; // shadowing
  }

  println!("not accessible outside block: {}", short_lived_binding)
}
```

## 5. Types

You can suffix numeric literals with their types:

```rust
fn main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
```

You can alias types:

```rust
// `NanoSecond` is a new name for `u64`.
type NanoSecond = u64;
type Inch = u64;

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5;
    let inches: Inch = 2;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
```

## 7. Expressions

Blocks are expressions too that are evaluated. It'll take the last line, or if a semicolon is used then it returns a `()`.

```rust
fn main() {
  let x = 5;

  let x_cubed = {
    let squared = x * x;
    squared * x
  };

  let z = {
    2 * x;
  };

  println!("{:?}", x_cubed); // 125
  println!("{:?}", z); // ()
}
```

## 8. Flow control

if/else are expressions and must return homogenous types across branches

```rust
fn main() {
  let n = -5;
  let abs_n = if n > 0 {
    n
  } else {
    -n
  };
  println!("{:?}", abs_n)
}
```

Loops: `loop`, `break`, `continue`. An expression can follow `break` and be the return value of the `loop`.

```rust
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
```

While: `while`

For loops: `for x in a..b`. `a..b` is from a to (b - 1). `a..=b` is a to b.

```rust
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
```

Switch/case: `match` and must be exhaustive. Can match values, destructure tuples, match enums, and structs.

```rust
fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);

    let pair = (0, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    // ENUMS
    
    #[allow(dead_code)]
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32)
    }

    let color = Color::Red;

    match color {
        Color::Red => println!("Red!"),
        Color::Blue => println!("Blue!"),
        Color::Green => println!("Green!"),
        Color::RGB(r, g, b) => println!("Red {} Blue {} Green {}", r, g, b)
    }

    // STRUCTS

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    // BINDING

    fn age() -> u32 {
        15
    }

    match age() {
        0 => println!("No birthdays yet"),
        n @ 1..=12 => println!("Child of age {}", n),
        n @ 13..=19 => println!("Teen of age {}", n),
        n => println!("Adult of age {}", n)
    }
}
```

## 9. Functions

### Methods

Functions given to objects, with references to other methods and data via `self`. Methods defined under `impl` block.

```rust
fn main() {
    struct Calculator {
        a: i32,
        b: i32
    }

    impl Calculator {
        // sugar for self: &Self
        fn add(&self) -> i32 {
            self.a + self.b
        }

        fn subtract(&self) -> i32 {
            self.a - self.b
        }

        // sugar for self: &mut Self
        fn add_to_both_values(&mut self, x: i32) {
            self.a += x;
            self.b += x;
        }
    }

    let mut calc = Calculator { a: 5, b: 2 };
    println!("5 + 2 = {}", calc.add());
    println!("5 - 2 = {}", calc.subtract());
    calc.add_to_both_values(4);
    println!("9 + 6 = {}", calc.add());
}
```

Static methods don't take `self` and are accessible via `MyClass::my_method`

```rust
fn main() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    // Implementation block, all `Point` methods go in here
    impl Point {
        // This is a static method
        // Static methods don't need to be called by an instance
        // These methods are generally used as constructors
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
    
        // Another static method, taking two arguments:
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    let origin = Point::origin();
    let p1 = Point::new(1.0, 2.0);
    println!("Point at origin: {:?}", origin);
    println!("Point at (1.0, 2.0): {:?}", p1);
}
```

Much longer example in `/methods` file...

## 12. Cargo

Start a new project: `$ cargo new foo`

Add dependencies, e.g. [`rand`](https://rust-random.github.io/book/guide-start.html):

```
[dependencies]
rand = "0.7.3"
```

Use in some code:

```
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let number = rand::thread_rng().gen_range(1, 11);
    println!("The number is: {}", number);
}
```

Building in two steps:

```
$ cargo build
$ ./target/debug/foo
```

Build and run:

```
$ cargo run
```
