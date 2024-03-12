fn main() {
    let mut x: u32 = 4;
    println!("x: {}", x);

    {
        let x = x - 2;
        println!("{}", x);
    }

    x = x + 5;
    println!("x now {}", x);

    let x = "hello";
    println!("{}", x);
}
