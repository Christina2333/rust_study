use cp10_trait::summary::Summary;

mod cp10_trait;

fn main() {

    let tweet = cp10_trait::summary::Tweet {
        username: String::from("christina"),
        content: String::from("想下班"),
        reply: false,
        reweet: false
    };
    println!("1 new tweet {}", tweet.summarize());

}
