use std::slice;
static mut COUNTER: u32 = 9;

#[cfg(test)]
mod test {

    use super::*;

    // 裸指针
    #[test]
    fn raw_pointer() {
        let mut num = 5;
        // 创建不可变裸指针，指针有效
        let r1 = &num as *const i32;
        // 创建可变裸指针，指针有效
        let r2 = &mut num as *mut i32;

        // 解引用裸指针时必须在unsafe块中
        unsafe {
            println!("r1 is {}", *r1);
            println!("r2 is {}", *r2);
            dangerous();
        }

        // 创建无效裸指针
        let address = 0x012345usize;
        let _r = address as *const i32;
    }

    // 把unsafe代码封装在安全函数中
    #[test]
    fn unsafe_function() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        // let (a, b) = r.split_at_mut(3);
        // let (a, b) = split_at_mut(r, 3);
        let (a, b) = split_at_mut_v2(r, 3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    // extern函数
    #[test]
    fn extern_function() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    // 修改和访问可变全局变量
    #[test]
    fn add_static() {
        add_to_count(1);

        unsafe {
            println!("counter={}", COUNTER);
        }
    }
}
// unsafe函数只能在unsafe块中调用
unsafe fn dangerous() {}

// 因为编译不通过，暂时注释掉
// fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let length = v.len();
//     assert!(length >= mid);
//     // 此处代码编译会报错，因为编译器认为我们可变借用了两次同一个切片，虽然可变借用同一切片的不同位置是不会引发问题的。
//     (&mut v[..mid], &mut v[mid..])
// }

fn split_at_mut_v2(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let length = v.len();
    assert!(mid <= length);
    let ptr = v.as_mut_ptr();

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), length - mid))
    }
}

// 调用C的abs函数
// 其中的"C"指明了外部函数使用的 应用二进制接口，被用来定义函数在汇编层面的调用方式。
extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}