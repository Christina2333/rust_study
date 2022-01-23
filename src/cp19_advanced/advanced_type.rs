#[cfg(test)]
mod test {

    #[test]
    fn alias_type() {
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let _f: Thunk = Box::new(|| println!("hi"));

        fn _takes_long_type(_f: Thunk) {}
        fn _returns_long_type() -> Thunk {
            Box::new(|| println!("hi"))
        }
    }
}
