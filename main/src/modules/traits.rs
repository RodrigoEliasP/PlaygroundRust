
pub fn largest_in_i32_array(number_list: &[i32]) {

    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
} 

