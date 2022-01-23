#[cfg(test)]
mod test {
    #[test]
    fn data_move() {
        let x = 5;
        let y = x;
        println!("x={}", x);
        println!("y={}", y);
    }
}