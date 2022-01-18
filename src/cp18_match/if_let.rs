#[cfg(test)]
mod test {
    use std::result::Result::Ok;
    use std::option::Option::Some;
    use super::*;

    #[test]
    fn if_let() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8,_> = "34".parse();

        // match favorite_color {
        //     Some(color) => {
        //         println!("Using your favorite color, {}, as the background", color);
        //     },
        //     None => {},
        // }

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            // if let允许同名变量替换，被替换的变量只有在花括号后的新作用域中才会有效。
            if age > 30 {
                println!("Using purple as background color");
            } else {
                println!("Using orange as background color");
            }
        } else {
            println!("Using blue as background color");
        }

    }

    #[test]
    fn while_lef() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // 当stack.pop()返回None时，退出循环
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    #[test]
    fn for_test() {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", index, value);
        }
    }

    #[test]
    fn test_print_coordinates() {
        let point = (3, 5);
        print_coordinates(&point);
    }

    #[test]
    fn match_named_val() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("matched, y = {:?}", y), // 此处y是match作用域的一个新变量，和上面的y无关，因此此时y会匹配5
            _ => println!("default case, x = {:?}", x),
        }
        // 离开match作用域后，x和y就是上面那个
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    #[test]
    fn match_range() {
        let x = 'c';
        match x {
            'a' ..= 'j' => println!("early ASCII letter"),
            'k' ..= 'z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    #[test]
    fn ignore_pattern() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            // 可以通过"_"匹配Some中的部分模式
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            },
            // 也可以通过"_"匹配全部模式
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);
    }

    #[test]
    fn ignore_pattern_1() {
        let s = Some(String::from("Hello!"));
        // 通过_s命名，虽然表示暂时不会使用，但是所有权已经转移到了_s中，因此打印s时会编译失败
        // if let Some(_s) = s {
        //     println!("found a string");
        // }
        // println!("{:?}", s);

        // 使用"_"不会进行绑定，因此可以打印s
        if let Some(_) = s {
            println!("found a string");
        }
        println!("{:?}", s);

    }

    #[test]
    fn match_guard() {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    #[test]
    fn match_named_val_guard() {
        let x = Some(10);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("matched, n = {:?}", n), // 通过match guard，实现match中变量和外部变量对比
            _ => println!("default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    #[test]
    fn match_guard_and_multi_pattern() {
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"), // if y同时作用与4，5，6
            _ => println!("no"),
        }
    }

    // @绑定
    #[test]
    fn banding() {
        let msg = Message::Hello {id: 5};
        match msg {
            Message::Hello {id: id_var @ 3..=7} => {
                println!("Found an id in range: {}", id_var);
            },
            Message::Hello {id: 10..=12} => {
                println!("Found an id in another range")
            },
            Message::Hello {id} => {
                println!("Found some other id: {}", id);
            },
        }
    }
}

pub fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

enum Message {
    Hello {id: i32}
}