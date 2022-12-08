#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
   if _first_list.eq(_second_list){
   return  Comparison::Equal
   }
   else if _first_list.iter().all(|list_item| _second_list.contains(list_item)) || _first_list == &[]{
    return Comparison::Sublist
   }
   else{
    return Comparison::Unequal
   }
}
