use std::fmt::Display;

pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 结构体的内部方法，需要显示指明生命周期
impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    // 应用了第三条省略生命周期规则，返回值和self生命周期相同
    pub fn announce_and_return_part(&self, ann: &str) -> &str {
        println!("Attention please {}", ann);
        self.part
    }
}
// 生命周期省略规则
// (1)每个入参都有自己的生命周期参数，即单参数有一个生命周期参数，双参数有两个不同的生命周期参数
// (2)当是有一个入参时，生命周期会被赋值给所有返回值
// (3)当入参有多个，且其中有一个是&self或者&mut self时，self的生命周期会赋值给输出参数`

// 综合trait、生命周期、范型
pub fn longest_with_ann<'a, T>(str1: &'a str, str2: &'a str, ann: T) -> &'a str
where T: Display {
    println!("display {}", ann);
    if str1.len() < str2.len() {
        str2
    } else {
        str1
    }
}