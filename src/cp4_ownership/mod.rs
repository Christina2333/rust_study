#[cfg(test)]
mod test {

    // 变量和数据的交互方式
    // (1)move
    #[test]
    fn data_move() {
        let x = 5;
        let y = x; // 实际上是复制
        println!("x={}", x);
        println!("y={}", y);

        let s = String::from("test");
        let s1 = s; // 此时s的所有权被释放。
        // println!("s={}", s); // 会报错
        println!("s1={}", s1);
    }
    // (2)clone
    #[test]
    fn data_clone() {
        let s = String::from("test");
        let s1 = s.clone();
        println!("s={}", s);
        println!("s1={}", s1);
    }
}