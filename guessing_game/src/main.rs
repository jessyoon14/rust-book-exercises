use std::io; // bring io library into scope
use std::cmp::Ordering; // enum (Less, Greater, Equal)
use rand::Rng; // trait, defines methods that random number generators implement
// crate: collection of Rust source code files
// binary crate: executable
// library crate: code intended to be used in other programs 

fn main() {
    println!("Guess the number!");

    // thread_rng: return random number generator local to current thread of execution and seeded by the OS
    // gen_range: defined by Rng trait. Take range expr as arg and generates a random number in the range
    let secret_number = rand::thread_rng().gen_range(1..101); // 1~100. equivalent to 1..=100

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // let: create variable. by default, variables are immutable
        // let mut: mutable
        // String::new(): new instance of string. new() is an associated function(i.e. static method) of String
        let mut guess = String::new();
    
        // stdin(): returns instance of std::io::Stdin
        // &: arg is reference (can access one piece of data without needing to copy that data into memory multiple times)
        // &mut: references are immutable by default, so need &mut to make mutable
        // read_line: read stdin, append to arg string, return io::Result (str arg must be mutable)
        // Result: enumerations (can have a fixed set of values -> variants)
        // Result enums: OK (successful value) || Err (why op failed) -> to encode error-handling info
        // expect: method of io::Result instance. if Err, crash & show arg message. If Ok, return value
        io::stdin()  // if std::io is not imported, can use std::io::stdin
             .read_line(&mut guess) 
            .expect("Failed to read line"); 
    
        // shadows previous value of guess (often used to convert value type)
        // trim(): remove \n. by default, guess would be "{num}\n"
        // parse(): parses string to number. Can create various number types, so need to specify u32
        // because of comparison, rust will infer that secret_number should also be u32
        // expect: parse returns Result type (in case parse returns error)
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // {}: placeholder
        println!("You guessed: {}", guess); 
    
        // cmp: compares two values. can be called on any comparable value
        // match: consists of arms
        // arm: pattern + code to be run if value fits the arm's pattern
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    
    
    }

     // rust has strong, static type system + type inference
}
