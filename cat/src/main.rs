extern crate cat_vg;

fn main() {
    println!("Hello, world!");
    cat_vg::test();
}

fn _thing_returning_closure() -> impl Fn(i32) -> bool {
    println!("here's a closure for you!");
    |x: i32| x % 3 == 0
}

