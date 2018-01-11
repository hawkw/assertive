#[macro_use]
extern crate assertive;

fn main() {
    let one = 1;
    let two = 2;
    println!("{}", assert_equal!(one, two));
    println!("{}", assert_equal!(one, 1));
}
