#[derive(Debug)] // 增加该注解可以保证值可以在断言失败时被打印，与之相似的还有PartialEq，
// 可以通过添加#[derive(PartialEq, Debug)]来添加判断和打印属性
pub struct Rectangle {
    pub width: u32,
    pub height: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}