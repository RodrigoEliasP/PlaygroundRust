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

fn test () {
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

fn main() {
    let (x1, x2) = quadratic_formula(1.0, -3.0, 2.0);
    println!("Quadratic formula roots are x1: {x1} and x2 {x2}");
    test();
}
