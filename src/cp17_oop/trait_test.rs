pub trait Draw {
    fn draw(&self);
}

// 屏幕
pub struct Screen {
    // 使用dyn关键字，因为不确定是哪个Draw的实现，没法确定占用字节大小
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw Button {:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw SelectBox {:?}", self);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn screen() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No")
                    ]
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("Ok"),
                }),
            ]
        };
        screen.run();
    }
}