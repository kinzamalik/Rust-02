fn main() {
    let s = String::from("Hello, Rust!"); // `s` owns the string
    take_ownership(s); // Ownership moves to the function, `s` is no longer valid here

    let x = 42; // `x` owns the integer
    makes_copy(x); // Since integers implement the `Copy` trait, `x` is still valid here

    println!("x is still accessible: {}", x);
}

fn take_ownership(some_string: String) {
    println!("String received: {}", some_string);
    // `some_string` goes out of scope here, memory is freed
}

fn makes_copy(some_integer: i32) {
    println!("Integer received: {}", some_integer);
    // `some_integer` goes out of scope, but since it’s a simple type, nothing special happens
}
