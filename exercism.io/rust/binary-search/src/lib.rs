pub fn find<U, T>(array: U, key: T) -> Option<usize>
where
    T: Ord,
    U: AsRef<[T]>,
{
    let mut array = &array
        .as_ref()
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &T)>>()[..];

    loop {
        let len = array.len();
        if len > 1 {
            let mid = len / 2;
            let (left, right) = array.split_at(mid);

            if right.first().unwrap().1 <= &key {
                array = right;
            } else {
                array = left;
            }
        } else {
            break;
        }
    }

    if let Some((index, value)) = array.first() {
        if value == &&key {
            return Some(*index);
        }
    }

    return None;
}
