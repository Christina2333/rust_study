enum Message {
    _Quit,
    _Move {x: i32, y: i32}, // 结构体
    Write(String),
    _ChangeColor(i32, i32, i32), // 元组结构体
}

// 同样可以在枚举中定义方法
impl Message {
    fn call(&self) {

    }
}
struct _QuitMessage; // 空结构体
struct _MoveMessage {
    x: i32,
    y: i32,
} // 结构体
struct _WriteMessage(String); // 元组结构体
struct _ChangeColorMessage(i32, i32, i32); // 元组结构体

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn message() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

}