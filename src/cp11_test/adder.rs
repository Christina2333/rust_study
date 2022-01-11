pub fn add2(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn print_and_return_10(i: i32) -> i32 {
    println!("got the value {}", i);
    10
}