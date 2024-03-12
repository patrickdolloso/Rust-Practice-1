fn main() {
    // tuple
    let mut tup: (i8, bool, char) = (1, true, 's');
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    // mutable
    tup.0 = 10;
    println!("{}, {}, {}", tup.0, tup.1, tup.2);


    // arrays
    let mut arr = [2, 0, 4, 4];
    arr[3] = 3;
    println!("{}", arr[3]);

}
