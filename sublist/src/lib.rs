#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if first_list.len() == 0 {
        return Comparison::Sublist;
    }

    if second_list.len() == 0 {
        return Comparison::Superlist;
    }

    let shortest_list;
    let longest_list;

    if first_list.len() > second_list.len() {
        shortest_list = second_list;
        longest_list = first_list;
    } else {
        shortest_list = first_list;
        longest_list = second_list;
    }

    for (_i, window) in longest_list.windows(shortest_list.len()).enumerate() {
        if window == shortest_list {
            if first_list.len() > second_list.len() {
                return Comparison::Superlist;
            } else {
                return Comparison::Sublist;
            }
        }
    }

    return Comparison::Unequal;
}
