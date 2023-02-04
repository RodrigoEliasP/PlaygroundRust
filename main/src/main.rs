#![allow(dead_code)]

fn quadratic_formula(a: f32, b:f32, c: f32) -> (f32, f32) {
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (x1, x2)
}
struct Owner{
    first_name: String,
    last_name: String,
}
struct Account {
    owner: Owner,
    balance: f64
}

impl Account {
    fn print_information(&self){
        println!(
            "Name {} last name {} balance {}", 
            self.owner.first_name, 
            self.owner.last_name, 
            self.balance
        );
    }
}

fn test() {
    let owner = Owner {
        first_name: String::from("Rodrigo"),
        last_name: String::from("Elias")
    };
    let account = Account {
        owner,
        balance: 3000.0
    };
    account.print_information();
}

fn mutability_data_types() {
    let number = String::from("34");
    let number: i8 = number.parse::<i8>().expect("") + 20;
    println!("the number is {number}");

    let mut overflow_wrap: i8 = (2_i8).pow(7)-1;


    overflow_wrap = i8::wrapping_add(overflow_wrap, 20);

    println!("the overflowed wrapped number is {overflow_wrap}");
}

fn control_flow() {
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

fn main() {
    // let (x1, x2) = quadratic_formula(1.0, -3.0, 2.0);
    // println!("Quadratic formula roots are x1: {x1} and x2: {x2}");
    // test();
    // mutability()
    control_flow();
}
