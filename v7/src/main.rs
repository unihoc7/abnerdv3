fn main() {
    let reference_to_nothing = dangling_reference();
}

fn dangling_reference() -> &String {
    let s = String::from("hello");

    &s
}