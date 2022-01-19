pub struct Guess {
    value: i32,
}
impl Guess {

    pub fn new(value: i32) -> Guess {
        if value < 1  {
            panic!("Guess value must be larger than 1, get {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than 100, get {}", value);
        }
        Guess {
            value
        }
    }

    // 相当于getter方法，返回私有变量value的值
    pub fn value(&self) -> i32 {
        self.value
    }
}