// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let (name,cat) = ("Furry McFurson", 3.5);
    let age/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
