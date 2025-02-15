fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15; // Changing the value of y
    println!("The value of y after modification is: {}", y);

    // Explicit type declaration
    let z: f64 = 3.14;
    println!("The value of z is: {}", z);

    // Shadowing variables
    let a = 5;
    let a = a + 1; // Shadowing the previous variable a
    let a = a * 2; // Shadowing again
    println!("The value of a after shadowing is: {}", a);

    // Constants
    const PI: f64 = 3.14159;
    println!("The value of PI is: {}", PI);

    // Tuples
    let person: (i32, &str) = (25, "John");
    println!("Age: {}, Name: {}", person.0, person.1);

    // Arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("The first element in the array is: {}", numbers[0]);

    // References
    let b = 10;
    let c = &b; // Reference to b
    println!("The value of c (reference to b) is: {}", c);

    // Type inference
    let inferred_type = 42; // Rust infers the type as i32
    println!("The value of inferred_type is: {}", inferred_type);
}
