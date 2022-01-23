#[cfg(test)]
mod test {
    use super::*;
    use std::mem;

    #[test]
    fn test_drop() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        // 显示释放c
        mem::drop(c);
        println!("CustomSmartPointers created.");
        // Drop::drop会在离开作用域时自动执行，变量丢弃顺序与创建顺序相反，先调用d的drop函数，再调用c的drop函数
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomPointer with data {} !", self.data);
    }
}