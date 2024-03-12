// 5. Collecting user input
/* 
Prelude:
- the list of things that Rust automatically imports into every Rust program.
- Kept as small as possible, and is focused on things, particularly traits used in almost every single Rust program
- Crate: Package/library of modules
*/

// using standard io crate 
use std::io;

fn main() {
    println!("Type input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

}
