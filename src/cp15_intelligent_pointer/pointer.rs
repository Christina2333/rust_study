
#[cfg(test)]
mod test {

    #[test]
    fn quote() {
        // 常规引用可以用过*获取值
        let x = 5;
        let y = &x;
        assert_eq!(x, 5);
        assert_eq!(*y, 5);

        // Box和普通引用使用方式一致，可以通过*获取值
        let boxy = Box::new(x);
        assert_eq!(*boxy, x);
    }

}