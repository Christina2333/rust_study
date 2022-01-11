use cp10_trait::summary::{Summary, notify, notify2, notify3};
use cp10_trait::pattern;
use cp10_trait::no_pattern;
use cp10_trait::partial::Pair;

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
    // 调用入参为trait的函数
    notify(&tweet);
    notify2(&tweet);
    notify3(&tweet, &tweet);

    // 使用带范型的函数比较大小
    let max = cp10_trait::pattern::find_largest(&list);
    println!("max:{}", max);
    let list = vec![34, 50, 25, 100, 65];
    println!("max:{}", cp10_trait::pattern::find_largest(&list));
    println!("max:{}", cp10_trait::pattern::find_largest_simple(&list));

    //
    let pair = cp10_trait::partial::Pair::new(10, 45);
    pair.cmp_display();

    // 生命周期
    println!("longest str={}", cp10_trait::no_pattern::longest_v2(String::from("zbv").as_str(), "zv"));
    // 可以通过编译，s1的生命周期>s2
    let s1 = String::from("test");
    {
        let s2 = "abc";
        println!("longest str={}", cp10_trait::no_pattern::longest_v2(&s1, s2));
    }
    // 调用longest_v2无法通过编译，因为result的生命周期应该和str2的生命周期相同，但实际result的生命周期大于str2的生命周期
    // 调用longest_v3则可以通过编译，因为指定了返回值的生命周期和str1相同
    let str1 = String::from("long str is long");
    let result;
    {
        let str2 = String::from("zwy");
        // result = cp10_trait::no_pattern::longest_v2(&str1, &str2);
        result = cp10_trait::no_pattern::longest_v3(&str1, &str2);
    }
    println!("longest str is {}", result);
    println!("result {}", cp10_trait::lifecycle::longest_with_ann(&str1, "测试", "测试哈哈"));

}
