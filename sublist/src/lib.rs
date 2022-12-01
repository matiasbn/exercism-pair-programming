#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug + std::fmt::Display>(
    first_list: &[T],
    second_list: &[T],
) -> Comparison {
    let first_list_string = to_string(first_list);
    let second_list_string = to_string(second_list);
    if is_equal(first_list_string.clone(), second_list_string.clone()) {
        Comparison::Equal
    } else if is_sublist(first_list_string.clone(), second_list_string.clone()) {
        Comparison::Sublist
    } else if is_superlist(first_list_string.clone(), second_list_string.clone()) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn to_string<T: PartialEq + std::fmt::Debug>(list: &[T]) -> String
where
    T: std::fmt::Display,
{
    let mut list_string = String::from("");
    for elem in list {
        list_string += elem.to_string().as_str();
    }
    list_string
}

fn is_equal(first_list: String, second_list: String) -> bool {
    first_list.len() == second_list.len() && first_list.as_str().contains(second_list.as_str())
}

fn is_sublist(first_list: String, second_list: String) -> bool {
    first_list.len() < second_list.len() && second_list.as_str().contains(first_list.as_str())
}

fn is_superlist(first_list: String, second_list: String) -> bool {
    first_list.len() > second_list.len() && first_list.as_str().contains(second_list.as_str())
}

#[test]
fn test_sublist() {
    let v: &[u32] = &[1];
    let v: &[u32] = &[1, 2];
    sublist(v, v);
}

#[test]
fn test_to_string() {
    let v: &[u32] = &[1, 2, 3];
    to_string(v);
}
