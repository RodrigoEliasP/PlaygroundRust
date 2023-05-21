pub fn control_flow() {
    let number = 23;

    if number < 5 {
        println!("Number is smaller than 5");
    } else {
        println!("Number is bigger than 5");
    }

    let mut number = 0;
    'external: loop {
        loop {
            if number % 10 <= 5 {
                println!("Number is {number}")
            }
            if number == 100 {
                break 'external;
            }
            number += 1;
            break;
        }
    }
}

pub fn mutability_data_types() {
    let number = String::from("34");
    let number: i8 = number.parse::<i8>().expect("") + 20;
    println!("the number is {number}");

    let mut overflow_wrap: i8 = (2_i8).pow(7)-1;


    overflow_wrap = i8::wrapping_add(overflow_wrap, 20);

    println!("the overflowed wrapped number is {overflow_wrap}");
}