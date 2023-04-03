use std::mem::swap;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let l1 = _first_list.len();
    let l2 = _second_list.len();

    match l1.cmp(&l2) {
        std::cmp::Ordering::Equal => {
            if _first_list == _second_list {
                return Comparison::Equal;
            } else {
                return Comparison::Unequal;
            }
        }
        std::cmp::Ordering::Less => {
            if l1 == 0 || _second_list.windows(l1).any(|window| _first_list == window) {
                return Comparison::Sublist;
            } else {
                return Comparison::Unequal;
            }
        }
        std::cmp::Ordering::Greater => {
            if l2 == 0 || _first_list.windows(l2).any(|window| window == _second_list) {
                return Comparison::Superlist;
            } else {
                return Comparison::Unequal;
            }
        }
    }
}
