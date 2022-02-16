# unsafe的超能力
（1）解引用裸指针
（2）调用unsafe的函数或方法
（3）访问或修改可变静态变量
（4）实现unsafe trait
（5）访问union字段

在unsafe代码块中，编译时，只有上述五种情况不会进行校验，其他借用检查仍然会执行。
为了隔离unsafe代码，可以将其封装到一个安全的抽象并提供安全API。

## 解引用裸指针(raw pointer)
悬垂指针(dangling pointer)：释放内存时，保留的指向内存的指针，即为悬垂指针。
如下方法会创建一个悬垂指针，因为方法中创建的s在离开方法时就不存在了，&s即为悬垂指针，该代码无法通过编译。
`
fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
let s = String::from("hello");
    &s
}
`
裸指针有两种：
*constant T：不可变裸指针，意味着指针解引用之后不能直接复制。
*mut T：可变裸指针
### 裸指针和智能指针的区别
（1）允许忽略借用规则，可以同时拥有不可变和可变指针，或多个指向相同位置的可变指针
（2）不保证指向有效的内存
（3）允许为空
（4）不能实现任何自动清理功能

### 裸指针如何使用
`
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
`
创建裸指针并不会不安全，使用裸指针的值才会引发问题，因此只有使用裸指针时才需要放在unsafe块中。

### 裸指针使用场景
（1）调用不安全函数或方法
例如标准库中。slice.as_mut_ptr()返回*mut T
（2）构建借用检查器无法理解的安全抽象
示例：cp19_advanced/unsafe_test.rs/split_at_mut_v2
引用C函数
`
extern "C" {
fn abs(input: i32) -> i32;
}
fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
`
## 调用不安全函数或方法

## 访问或修改可变静态变量
常量和静态变量不同，静态变量拥有固定的地址，常量可以通过复制来提供使用。
全局变量（静态变量），访问不可变静态变量是安全的，访问和修改可变静态变量是不安全的。
`
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn main() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
`

## 实现unsafe trait
当trait中包含unsafe的方法时，需要在trait前面增加unsafe
`
fn main() {
    unsafe trait Foo {
        // methods go here
    }
    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
`
如果一个类型A完全由实现了Send/Sync的类型组成，那么编译器会自动为A实现Send/Sync。
如果类型A中存在未实现Send/Sync的类型，例如裸指针，但是希望A实现Send/Sync，需要标注unsafe。
## 访问union字段
rust调用C时会出现、