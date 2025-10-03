#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist_of(smaller: &[i32], bigger: &[i32]) -> bool {
    smaller.is_empty() || bigger.windows(smaller.len()).any(|w| w == smaller)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist_of(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist_of(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
