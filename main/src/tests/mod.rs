
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
                assert_eq!(list.get_size(), 3);
            }
            #[test]
            fn find_sould_work(){
                let mut list: LinkedList<i32> = LinkedList::new();
                list.insert(10);
                list.insert(2);
                list.insert(20);
                let result_ten = list.find(
                    |&i: &i32| -> bool { i == 10 } 
                );
                assert_eq!(result_ten, Ok(&10));
                let result_twenty = list.find(
                    |&i: &i32| -> bool { i == 20 } 
                );
                assert_eq!(result_twenty, Ok(&20));

                let not_found_error = list.find(
                    |&i: &i32| -> bool { i == 21 } 
                );
                assert_eq!(not_found_error, Err("Not found".to_owned()));

            }
        }
    }
}