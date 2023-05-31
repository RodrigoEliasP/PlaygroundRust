
use crate::modules::structs::{ LinkedList };

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    mod structs {
        use super::*;
        #[cfg(test)]
        mod linked_list {
            use super::*;
            #[test]
            fn insert_should_work(){
                let mut list: LinkedList<i32> = LinkedList::new();
                list.insert(10);
                list.insert(2);
                list.insert(20);
                assert_eq!(list.to_string(), "10;2;20".to_owned());
            }
        }
    }
}