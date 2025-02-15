fn main(){
    let my_name = "Kinza";
    match my_name{
        "Kinza" => println!("That's my name"),
        "Alice" => println!("That's not my name"),
        "Bob" => println!("Hello Bob!"),
        _ => println!("Somthin else")
    }
}