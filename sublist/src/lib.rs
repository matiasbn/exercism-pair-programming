use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Display>(_first_list: &[T], _second_list: &[T]) -> Comparison {
   let first_list_string: String = _first_list.iter().map(|x| x.to_string()).collect::<String>();
   let second_list_string: String = _second_list.iter().map(|x| x.to_string()).collect::<String>();

   if is_equal(_first_list, _second_list){
   return  Comparison::Equal
   }
   else if is_sublist(&first_list_string, &second_list_string) {
    return Comparison::Sublist
   }
   else if is_superlist(&first_list_string, &second_list_string) {
    return Comparison::Superlist
   }
   else{
    return Comparison::Unequal
   }
}

pub fn is_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool{
    first_list.eq(second_list)
}

pub fn is_sublist(first_list: &String, second_list: &String) -> bool{
    // first_list.iter().all(|list_item| second_list.contains(list_item)) || first_list == &[]
    second_list.contains(first_list) || first_list.len() == 0
}

pub fn is_superlist(first_list: &String, second_list: &String) -> bool{
    // second_list.iter().all(|list_item| first_list.contains(list_item)) || second_list == &[]
    first_list.contains(second_list) || second_list.len() == 0
}

