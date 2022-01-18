// trait State {
//     fn approve(self: Box<Self>) -> Box<dyn State>;
// }
//
// struct Draft {}
// impl State for Draft {
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
// }
//
// struct PendingReview {}
// impl State for PendingReview {
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// }
//
// struct Published {}
// impl State for Published {
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// }
//
// struct Post {
//     content: String,
//     state: Box<dyn State>
// }
// impl Post {
//     fn new() -> Box<Post> {
//         Box::new(
//             Post {
//                 content: String::new(),
//                 state: Box::new(Draft {})
//             }
//         )
//     }
//     fn add_content(&mut self, content: String) {
//         self.content.push_str(&content);
//     }
//     fn approve(&mut self) {
//         let state = self.state.approve();
//         self.state = state;
//     }
// }
// #[cfg(test)]
// mod test {
//
//     use super::*;
//
//     #[test]
//     fn works() {
//         let mut post = Post::new();
//         let content = String::from("test content");
//         post.add_content(content);
//
//         post.approve();
//
//         post.approve();
//
//     }
// }
