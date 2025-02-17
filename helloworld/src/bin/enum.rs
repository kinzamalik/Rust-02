enum Direction{
    Left,
    Right,
    Up,
    Down,
}

fn main(){
    let go = Direction::Left;
    match go {
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down")
    }
}