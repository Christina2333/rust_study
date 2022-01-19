// 一个简化后的vec!宏
#[macro_export]  // 当该宏所在的包被引入时可用。没有这个标注的宏不能被引入
macro_rules! vec {  // 使用macro_rules!定义宏，vec是宏的名称
// ()中是模式，如果匹配到该模式会执行=>后面的代码。","表示模式中会包含的逗号，"*"表示前面的部分会重复0个或多个
// 如果输入是vec![1,2,3]，逗号分隔，因此[$x:expr]会匹配1、2、3，匹配到的值存储到$x中
  ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
          $(
              temp_vec.push($x);
           )*
          temp_vec
        }
    };
}