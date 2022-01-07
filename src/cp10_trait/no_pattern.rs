pub fn find_largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &i in list.iter() {
        if max < i {
            max = i;
        }
    }
    return max;
}