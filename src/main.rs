use travis_test::add_two;

fn main() {
    let a = 4u32;
    println!("{}", hello(a));
    if a > 0 {
  println!("{}", add_two(1));
    }
}

fn hello(a: u32) -> bool {
    if a > 0 {
        return true;}
    false
}
