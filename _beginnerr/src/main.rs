// An example of ownership in action.

fn main() {
    let r1: String = String::from("Rust");

    println!("r1 is: {r1}");
}

fn print_string(p1: String) {
    println!("{p1}");
}