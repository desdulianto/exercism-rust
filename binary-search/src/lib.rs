use std::cmp::Ordering;

pub fn find<T, TSlice>(array: TSlice, key: T) -> Option<usize>
where
    T: Ord,
    TSlice: AsRef<[T]>,
{
    let array = array.as_ref();
    let mut left = 0 as isize;
    let mut right = array.len() as isize - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if let Some(n) = &array.get(mid as usize) {
            match key.cmp(n) {
                Ordering::Equal => return Some(mid as usize),
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid - 1,
            }
        } else {
            return None;
        }
    }
    None
}
