use std::fmt::{Display, Formatter, Error, Debug};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trait_test() {
        let tweet = Tweet {
            username: String::from("christina"),
            content: String::from("想下班"),
            reply: false,
            reweet: false
        };
        println!("1 new tweet {}", tweet.summarize());
        // 调用入参为trait的函数
        notify(&tweet);
        notify2(&tweet);
        notify3(&tweet, &tweet);
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub reweet: bool,
}

impl Tweet {
    pub fn display(&self) -> String {
        format!("username: {}, content : {}, reply: {}, reweet: {}", self.username, self.content, self.reply, self.reweet)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// 参数为trait，为了保证item经过调用之后可用，采用&
// 当入参为多个trait时，可以采用"+" item: &(impl Summary + Display)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 如果入参为多个trait，也可以使用where
pub fn notify_where<T, U>(item1: &T, item2: &U)
where T: Display + Clone,
U: Clone + Debug
{}

// 另一种方式使用trait作为参数
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// 如果想要两个参数均为一个类型的Summary，可以采用范型的方式，非范型方式无法保证两个入参类型一致
pub fn notify3<T: Summary>(item: &T, item2: &T) {
    println!("two Summary param! {}, {}", item.summarize(), item2.summarize());
}
// 返回值中使用trait,但是不支持根据入参的不同，返回Tweet或者NewsArticle，ch17会进行讲解
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("xxxx"),
        reply: false,
        reweet: false
    }
}