use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (longer, shorter, comparison) = match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Equal => (_first_list, _second_list, Comparison::Equal),
        Ordering::Greater => (_first_list, _second_list, Comparison::Superlist),
        Ordering::Less => (_second_list, _first_list, Comparison::Sublist),
    };

    let is_sublist = (0..=(longer.len() - shorter.len())).any(|i| {
        longer[i..]
            .iter()
            .zip(shorter.iter())
            .all(|x| x.0 == x.1)
    });

    if is_sublist {
        comparison
    } else {
        Comparison::Unequal
    }
}
