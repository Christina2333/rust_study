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