智能指针


内部可变性：不可变类型暴露出改变其内部值的API。

#### 智能指针

#### 内部可变性
在使用不可变引用时，可以改变其内部数据。即可以获取不可变值的可变引用。通过*unsafe*来模糊rust的可变性和借用规则。

#### newtype模式

##### 在外部类型上实现外部trait
例如：想在Vec<T>上实现Display（孤儿原则禁止）
可以创建包含Vec<T>的实例Wrapper，在Wrapper上实现Display
`
use std::fmt;
    struct Wrapper(Vec<String>);  // 包含Vec<String>元组
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", ")) // self.0即为Vec<String>
        }
    }
    fn main() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
`
如果希望新类型拥有内部类型的每一个方法：可以为封装类型实现Deref trait，这样可以通过&xxx获取原结构体。
如果只希望内部类型的部分方法：可以手动实现。
