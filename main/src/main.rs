#![allow(dead_code)]
#![allow(unused_imports)]
mod modules;
mod tests;

use std::{time::Instant};
use modules::quadratic_formula::quadratic_formula;
use modules::traits::largest_in_i32_array;
use modules::structs::{structs_example, LinkedList};
use modules::iterables::searches_test;
use modules::basics::{ control_flow, mutability_data_types };

fn main() {
    // let (x1, x2) = quadratic_formula(1.0, -3.0, 2.0);
    // println!("Quadratic formula roots are x1: {x1} and x2: {x2}");
    // test();
    // mutability()
    //searches_test();
    // largest_in_i32_array(&[1,2,3,4,5,6,7,8]);
    let mut linked_list: LinkedList<i32> = LinkedList::new();

    linked_list.insert(30);
    linked_list.insert(32);
    linked_list.insert(33);
    linked_list.insert(34);
    linked_list.insert(35);
    println!("{}",linked_list);

}
