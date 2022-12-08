#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
   if is_equal(_first_list, _second_list){
   return  Comparison::Equal
   }
   else if is_sublist(_first_list, _second_list) {
    return Comparison::Sublist
   }
   else if is_superlist(_first_list, _second_list) {
    return Comparison::Superlist
   }
   else{
    return Comparison::Unequal
   }
}

pub fn is_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool{
    first_list.eq(second_list)
}

pub fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool{
    // first_list.iter().all(|list_item| second_list.contains(list_item)) || first_list == &[]
    
}

pub fn is_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool{
    second_list.iter().all(|list_item| first_list.contains(list_item)) || second_list == &[]
}

