fn main() {
    println!("Hello, world!");
    test_one();
    let result = add_numbers(20,30);
    println!("{}", result);

    let number = {
        let i = 3;
        i + 2
    };
    println!("{}", number);
}

fn test_one() {
    println!("Test has been called");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    return x + y; 
}
