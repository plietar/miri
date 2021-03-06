// FIXME: The normal `vec!` macro is currently broken in Miri because it hits the
// std::vec::SetLenOnDrop code and Miri doesn't call destructors yet.
macro_rules! miri_vec {
    ($($e:expr),*) => ({
        let mut v = Vec::new();
        $(v.push($e);)*
        v
    });
}

fn make_vec() -> Vec<u8> {
    let mut v = Vec::with_capacity(4);
    v.push(1);
    v.push(2);
    v
}

fn make_vec_macro() -> Vec<u8> {
    miri_vec![1, 2]
}

fn make_vec_macro_repeat() -> Vec<u8> {
    miri_vec![42, 42, 42, 42, 42]
}

fn vec_into_iter() -> u8 {
    miri_vec![1, 2, 3, 4]
        .into_iter()
        .map(|x| x * x)
        .fold(0, |x, y| x + y)
}

fn vec_reallocate() -> Vec<u8> {
    let mut v = miri_vec![1, 2];
    v.push(3);
    v.push(4);
    v.push(5);
    v
}

fn main() {
    assert_eq!(vec_reallocate().len(), 5);
    assert_eq!(vec_into_iter(), 30);
    assert_eq!(make_vec().capacity(), 4);
    assert_eq!(make_vec_macro(), [1, 2]);
    assert_eq!(make_vec_macro_repeat(), [42; 5]);
}
