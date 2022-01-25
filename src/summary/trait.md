trait类似与接口和抽象类的组合。可以定义方法，同时可以为方法增加默认实现。

#### 使用方式

#####（一）可以作为入参

假设Summary为一个trait，里面有一个summarize方法。
`pub fn notify(item: impl Summary) {
    println!("Breaking news {}", item.summarize());
}`
Trait Bound语法表示上述例子：
`pub fn notify<T: Summary>(item: T) {}`
`pub fn notify<T: Summary + Display>(item: T) {}`

`fn some_function<T: Display + Clone, U: Clone + Debug>(t:T, u:U) -> i32{}`
`fn some_function<T, U>(t:T, u:U) -> i32
where T: Display + Clone,
      U: Clone + Debug {}`

#####（二）可以作为函数返回值
`fn return_summarizable() -> impl Summary {}`


#####（三）使用trait bound有条件地实现方法
`fn main() {
    use std::fmt::Display;
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}`

##### （四）trait对象
即一个结构体中有属性为trait

###### trait对象执行动态分发
单态化：通过填充编译时使用的具体类型，将通用代码替换为特定代码的过程。例如范型代码，rust就是通过单态化来保证效率。
静态分发：编译器在编译时就确定调用的具体方法
动态分发：编译器无法在编译时确定调用的是哪个方法，会额外生成一些代码以便在运行时找出希望调用的方法。
因为编译器无法知晓能够用于trait对象的具体类型，所以它无法在编译时确定需要调用哪个类型的哪个方法，因此trait对象执行动态分发。

###### trait对象要求对象安全
只有对象安全的trait才能组成trait对象。
对象安全的trait要求其下方法满足如下两个规则：
（1）方法的返回类型不是Self（Self是别名，指向实现当前trait或方法的具体类型）
（2）方法中不包含任何范型参数
标准库中的Clone trait不是对象安全的，因此Clone trait无法组成trait对象。
`
pub trait Clone {
    fn clone(&self) -> Self;
}
`

###### trait的孤儿原则
被称为相干性（coherence）的程序属性的一部分。
目的：为了确保其他人编写的代码不会破坏你的代码。
内容：只有当trait或者要实现trait的类型位于crate的本地作用域时，才能为该类型实现trait。
举例：不能在自定义crate中为Vec<T>实现Display trait，因为Display和Vec<T>都定义在标准库中，不位于自定义crate本地作用域。
打破：特殊情况下可以cp19！！！



#### 高级trait
与trait相关的关联类型，默认类型参数，完全限定语法，supertraits、newtype模式
