#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_pattern_find_largest() {
        let list = [3, 6, 1, 10];
        let max = find_largest(&list);
        println!("max={}", max);
    }

    #[test]
    fn lifecycle() {
        // 生命周期
        println!("longest str={}", longest_v2(String::from("zbv").as_str(), "zv"));
        // 可以通过编译，s1的生命周期>s2
        let s1 = String::from("test");
        {
            let s2 = "abc";
            println!("longest str={}", longest_v2(&s1, s2));
        }
        // 调用longest_v2无法通过编译，因为result的生命周期应该和str2的生命周期相同，但实际result的生命周期大于str2的生命周期
        // 调用longest_v3则可以通过编译，因为指定了返回值的生命周期和str1相同
        let str1 = String::from("long str is long");
        let result;
        {
            let str2 = String::from("zwy");
            // result = longest_v2(&str1, &str2);
            result = longest_v3(&str1, &str2);
        }
        println!("longest str is {}", result);
    }
}


pub fn find_largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &i in list.iter() {
        if max < i {
            max = i;
        }
    }
    return max;
}

// 这段函数编译会报错，因为rust的【借用检查器】无法判断返回值指向的是str1还是str2的引用
// 其实用于也无法进行判断，这样【借用检查器】就无法对引用和数据的有效关系进行判断了，为了解决这个问题引入了范型生命周期参数
// pub fn longest(str1: &str, str2: &str) -> &str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// 为了处理上述情况，增加了范型生命周期引用"'a"，含义是：返回引用的生命周期应该与str1和str2生命周期中较短的那个相同
pub fn longest_v2<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() < str2.len() {
        str2
    } else {
        str1
    }
}

// 如果想要指定返回值的生命周期和str1相同，则无需指定str2的生命周期，如下所示
// 函数返回值的生命周期一定要与至少一个入参的生命周期关联
pub fn longest_v3<'a>(str1: &'a str, str2: &str) -> &'a str {
    str1
}