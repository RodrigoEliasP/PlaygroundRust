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

pub fn structs_example() {
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