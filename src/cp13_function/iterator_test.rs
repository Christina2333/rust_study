#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {

    shoes.into_iter().filter(|x| x.size == shoe_size ).collect()
    // shoes.iter().filter(|x| x.size == shoe_size ).collect()

}

#[cfg(test)]
mod shoe_test {
    use super::*;

    #[test]
    fn iterator_filter() {
        let shoes = vec![
            Shoe {
                size: 10, style : String::from("sneaker")
            },
            Shoe {
                size: 13, style: String::from("sandle")
            },
            Shoe {
                size: 10, style:String::from("boot")
            }
        ];
        let res = shoes_in_my_size(shoes, 10);
        println!("res={:?}", res);
    }
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count >= 6 {
            None
        } else {
            Some(self.count)
        }
    }
}

#[cfg(test)]
mod counter_test {
    use super::*;

    #[test]
    fn counter_iterator() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        // zip函数会在其中一个迭代器达到none时结束
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(sum, 18);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn iter() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    #[test]
    fn iterator_demonstration() {
        let mut v1 = vec![1, 2, 3];
        // 此处迭代器必须是可变的，因为next会改变迭代器内部记录位置的状态
        // 单测iter()未要求迭代器可变，原因是循环取得了v1_iter的所有权并在内部使得其可变？？？
        let mut v1_iter = v1.iter();
        // 同时，这里next返回的是指向动态数组中元素的不可变引用，如果想要改变元素值。可以使用iter_mut
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        println!("v1:{:?}", v1);

        let mut iter_mut = v1.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), None);
        println!("v1:{:?}", v1);

        // 如果想要返回元素本身，可以使用into_iter，但是循环之后v1就不可被访问了
        let mut into_iter = v1.into_iter();
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), None);
        // println!("v1:{:?}", v1);
    }

    // 有些方法会消耗迭代器，即迭代器使用之后不可再用
    #[test]
    fn iterator_sum() {
        let v = vec![1, 2, 3];
        let iter = v.iter();
        let total: i32 = iter.sum();
        println!("total={}", total);
        // let total1: i32 = iter.sum();
        // println!("total1={}", total1);
    }

    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        //
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}