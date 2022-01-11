use crate::*;
use super::*;

// 通过cargo test执行全部单测
// cargo test默认使用多线程并发执行单测，可以通过cargo test -- --test-threads=1 设置线程数为1
// cargo test -- --nocapture 运行测试时禁用输出功能
// cargo test 函数名，可以执行单个单测
// 例如：cargo test this，会运行两个名称中含"this"的单测
// cargo test -- --ignored，执行被忽略的单测
// 测试一般分为【单元测试】和【集成测试】，一般只对单元测试标注#[cfg(test)]，因为集成测试在独立的目录中

#[cfg(test)] // 此标注可以在执行cargo test时编译和运行部分测试代码，并在cargo build时剔除，
// cfg为configuration，说明只在test配置时才将下面代码纳入编译范围
mod tests {

    #[test]  // 将当前函数标记为一个测试
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = crate::cp5_struct::structs::Rectangle {
            width: 8,
            height: 7
        };
        let smaller = crate::cp5_struct::structs::Rectangle {
            width: 4,
            height: 3
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = crate::cp5_struct::structs::Rectangle {
            width: 8,
            height: 7
        };
        let smaller = crate::cp5_struct::structs::Rectangle {
            width: 4,
            height: 3
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn adds_two() {
        assert_eq!(4, crate::cp11_test::adder::add2(2));
    }

    #[test]
    fn greeting_with_name() {
        let name = super::adder::greeting("test");
        // 如果断言结果为false，返回后面的提示信息
        assert!(name.contains("test"), "Greeting did not contains name, value: {}", name);
    }

    // 用should_panic检查panic
    use crate::cp9_exception::guess::Guess;
    use super::adder::print_and_return_10;

    #[test]
    #[should_panic(expected = "Guess value must be less than 100")]
    // 说明测试会抛出异常，加上该注解则单测不会失败，expected表示期望抛出的异常信息
    fn guess_greater_than_100() {
        let res = Guess::new(200);
    }

    // 测试返回Result
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 plus 2 not equals 4"))
        }
    }

    // 由于函数print_and_return_10会输出信息，导致结果混乱，可以使用cargo test -- --nocapture禁止输出功能
    #[test]
    fn this_test_will_pass() {
        let res = print_and_return_10(4);
        assert_eq!(10, res);
    }

    #[test]
    #[ignore] // 设置忽略该测试用例
    fn this_test_will_fail() {
        let res = print_and_return_10(8);
        assert_eq!(5, res);
    }
}


