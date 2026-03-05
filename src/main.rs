use std::io;
use rand::Rng;

const MAX_RANGE: u32 = 100;

fn main() {
    println!("Hello, world!");
    // guessing_game();
    // mutable_variable();
    // array_type();
    // basic_move_semantics();
    mutable_borrow_and_shared_reference();
}

fn guessing_game() {
    println!("Welcome to the guessing game!");
    println!("You have 10 tries to guess the number between 1 and 100.");
       // loop 10 times are till the user guesses the number
    for _i in 0..1 {
        match guess_number().cmp(&random_number()) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                return;
            },
        };
    }
}

// Function to get the user's guess
// returns the user's guess as a u32
fn guess_number() -> u32 {
    println!("Guess the number!");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    // loop till the user enters a valid number
    loop {
        match guess.trim().parse::<u64>() {
            Ok(num) => return num as u32,
            Err(ex) => {
                let exMsg = ex.to_string();
                println!("Please type a number!");
                guess.clear();
                io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            }
        }
    }
}

// Generate a random number between 1 and 100
// returns the random number as a u32
fn random_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=MAX_RANGE);
    secret_number
}


///
/// let x = String::from("hello");
// let y = x;  // MOVE
//
// Heap (no GC!):
// ┌─────────────┐
// │   "hello"   │ ←── ONLY y owns this, x is invalid
// └─────────────┘
//       ↑
//       │
//       y
// (single owner, deterministic cleanup)
// ```
//
// ---
//
// ## Summary Diagram
// ```
// ┌─────────────────────────────────────────────────────────┐
// │                  OWNERSHIP PATTERNS                      │
// ├─────────────────────────────────────────────────────────┤
// │                                                          │
// │  MOVE: let y = x                                        │
// │  ┌───┐          ┌───┐                                   │
// │  │ x │  ───>    │ y │  ──────> [heap data]             │
// │  └───┘          └───┘                                   │
// │  DEAD           OWNER                                    │
// │                                                          │
// │  IMMUTABLE BORROW: let y = &x                           │
// │  ┌───┐                                                   │
// │  │ x │  ──────────────> [heap data]                     │
// │  └───┘          ↑                                        │
// │  OWNER          │                                        │
// │  ┌───┐          │                                        │
// │  │ y │  ────────┘                                        │
// │  └───┘                                                   │
// │  READER                                                  │
// │                                                          │
// │  MUTABLE BORROW: let y = &mut x                         │
// │  ┌───┐                                                   │
// │  │ x │  ──────────────> [heap data]                     │
// │  └───┘          ↑                                        │
// │  FROZEN         │ (exclusive)                            │
// │  ┌───┐          │                                        │
// │  │ y │  ────────┘                                        │
// │  └───┘                                                   │
// │  EXCLUSIVE WRITER                                        │
// │                                                          │
// └─────────────────────────────────────────────────────────┘

fn mutable_variable() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
}

fn array_type() -> () {
    let x = [1, 2, 3];
    println!("Array length: {}", x.len());
    for (index, element) in x.iter().enumerate() {
        println!("Index: {}, Value: {}", index, element);
    }
    println!("Array value: {}", x[0]);
}

fn basic_move_semantics() -> (){
    let x = String::from("hello");
    let y = x;
    // println!("The value of x is: {}", x); // Not valid; borrowed by y
    println!("The value of y is: {}", y);
}

///
/// Stack:                          Heap:
// ┌─────────────┐                ┌─────────────────┐
// │     x       │                │                 │
// │  ┌────────┐ │     ╔═════════>│  'h' 'e' 'l'   │
// │  │ ptr    │─┼─────╝          │  'l' 'o'       │
// │  │ len: 5 │ │     ║          │                 │
// │  │ cap: 5 │ │     ║          │  (5 bytes)     │
// │  └────────┘ │     ║          └─────────────────┘
// ├─────────────┤     ║
// │     y       │     ║
// │  ┌────────┐ │     ║
// │  │  ref   │─┼─────╝  (points to x on stack)
// │  └────────┘ │
// ├─────────────┤
// │     z       │
// │  ┌────────┐ │
// │  │  ref   │─┼─────╝  (also points to x on stack)
// │  └────────┘ │
// └─────────────┘
//
// Key:
// - x OWNS the heap data
// - y and z are REFERENCES to x (fat pointers)
// - All three can read the same heap data
// - Multiple borrows = multiple pointers to the OWNER
// - Heap data stays in one place
///
fn immutable_borrow_and_shared_reference() -> () {
    let x = String::from("hello");
    {
        let y =  &x;
        println!("The value of y is: {}", y);
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
}


///
/// let mut x = String::from("hello");
// let y = &mut x;
// y.push_str(" world");
// ```
//
// ### Memory Layout:
// ```
// BEFORE mutable borrow:
//
// Stack:                          Heap:
// ┌─────────────┐                ┌─────────────────┐
// │     x       │                │                 │
// │  ┌────────┐ │     ╔═════════>│  'h' 'e' 'l'   │
// │  │ ptr    │─┼─────╝          │  'l' 'o'       │
// │  │ len: 5 │ │                │                 │
// │  │ cap: 5 │ │                │  (5 bytes)     │
// │  └────────┘ │                └─────────────────┘
// └─────────────┘
//
// AFTER let y = &mut x:
//
// Stack:                          Heap:
// ┌─────────────┐                ┌─────────────────┐
// │     x       │                │                 │
// │  ┌────────┐ │     ╔═════════>│  'h' 'e' 'l'   │
// │  │ ptr    │─┼─────╝          │  'l' 'o'       │
// │  │ len: 5 │ │     ║          │                 │
// │  │ cap: 5 │ │     ║          │  (5 bytes)     │
// │  └────────┘ │     ║          └─────────────────┘
// │   (FROZEN)  │     ║
// ├─────────────┤     ║
// │     y       │     ║
// │  ┌────────┐ │     ║
// │  │ mut ref│─┼─────╝  (exclusive reference to x)
// │  └────────┘ │
// └─────────────┘
//
// AFTER y.push_str(" world"):
//
// Stack:                          Heap:
// ┌─────────────┐                ┌─────────────────────────┐
// │     x       │                │                         │
// │  ┌────────┐ │     ╔═════════>│  'h' 'e' 'l' 'l' 'o'   │
// │  │ ptr    │─┼─────╝ ║        │  ' ' 'w' 'o' 'r' 'l'   │
// │  │ len: 11│ │  (updated      │  'd'                   │
// │  │ cap: 11│ │   via y)       │                         │
// │  └────────┘ │       ║        │  (11 bytes)            │
// │   (FROZEN)  │       ║        └─────────────────────────┘
// ├─────────────┤       ║
// │     y       │       ║
// │  ┌────────┐ │       ║
// │  │ mut ref│─┼───────╝
// │  └────────┘ │
// └─────────────┘
//
// Key:
// - y is exclusive mutable reference to x
// - y can modify the heap data through x's pointer
// - x is frozen (compiler prevents access)
// - Only ONE mutable borrow allowed
// - x's metadata (len, cap) gets updated through y
fn mutable_borrow_and_shared_reference() -> () {
    let mut x = String::from("hello");
    {
        let y = &mut x;
        println!("The value of y before push is: {}", y);
        y.push_str(" world");
        println!("The value of y after push is: {}", y);
    }
    println!("The value of x after lease expired is: {}", x);
}