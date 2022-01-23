#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn blog_test() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");

        assert_eq!(post.content(), "");

        post.request_review();
        assert_eq!(post.content(), "");

        post.approve();
        assert_eq!(post.content(), "I ate a salad for lunch today");
    }
}

trait State {
    // 状态变更，会消耗当前状态返回新的状态，会消耗所有权，返回新的状态
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // 执行审批流程
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct Post {
    // 因为把State定义为trait，所以只能用Box<dyn State>引用
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    // 新建文章，初始状态为Draft
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    // 添加文字
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // 展示文章内容，只有Published状态才能展示信息，否则为空
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    // 请求审核
    pub fn request_review(&mut self) {
        // take()获取Some中的值，并在原来的位置留下None
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        // take()获取Some中的值，并在原来的位置留下None
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}