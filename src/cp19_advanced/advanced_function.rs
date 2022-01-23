#[cfg(test)]
mod test {

    #[test]
    fn fun_pointer() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        let answer = do_twice(add_one, 5);
        println!("answer={}", answer);
    }

    // 闭包和函数作为参数
    #[test]
    fn fun_and_closure() {
        // 可以接收闭包
        let list_of_numbers = vec![1, 2, 3];
        let _list_of_strings: Vec<String> = list_of_numbers
            .iter()
            .map(|i| i.to_string())
            .collect();
        // 可以接收函数，注意map中如果传入的是ToString::to_string()会报错
        let _list_of_string2: Vec<String> = list_of_numbers
            .iter()
            .map(ToString::to_string)
            .collect();
    }

    #[test]
    fn construct_closure() {
        enum Status {
            Value(u32),
            _Stop
        }
        // (0u32..20)表示(0..20)的元组数据u32表示0的数据类型
        let _list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
        // 等价于上面的写法
        let _list_of_status: Vec<Status> = (0u32..20).map(|x| Status::Value(x)).collect();
    }
}