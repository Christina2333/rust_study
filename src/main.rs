use cp10_trait::summary::Summary;
use cp10_trait::pattern;
use cp10_trait::no_pattern;

mod cp10_trait;

fn main() {
    // (1)没有范型的函数
    let list = [3, 6, 1, 10];
    let max = cp10_trait::no_pattern::find_largest(&list);
    println!("max={}", max);

    // (2)struct的范型
    let integer = cp10_trait::pattern::Point::new(1, 3);
    let float = cp10_trait::pattern::Point::new(1.9, 3.4);
    println!("float.x={}", float.x());
    println!("float.distance={}", float.distance_from_origin());

    let p1 = cp10_trait::pattern::Point2::new(1, "c");
    let p2 = cp10_trait::pattern::Point2::new("a", 3.9);
    let p3 = p1.mixup(p2);
    println!("p3.x={}, p3.y={}", p3.x(), p3.y());


    // (3)trait
    let tweet = cp10_trait::summary::Tweet {
        username: String::from("christina"),
        content: String::from("想下班"),
        reply: false,
        reweet: false
    };
    println!("1 new tweet {}", tweet.summarize());

}
