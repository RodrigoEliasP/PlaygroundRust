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

struct Node< T> {
    next: Option<Box<Node<T>>>,
    data: T
}

struct LinkedList<T> {
    start: Option<Box<Node<T>>>,
    qtd: u32,
}

impl<T> LinkedList<T> {
    pub const fn new<'a>() -> Self {
        
        LinkedList { start: None, qtd: 1 }
    }
    fn new_node(&mut self,data: T) -> Node<T> {
        Node {
            next: None,
            data
        }
    }
    pub fn insert(&mut self, data: T) -> u32 {
        let index = self.qtd;
        self.qtd += 1;

        let node = self.new_node(data);

        if self.start.is_none(){
            self.start = Option::Some(Box::new(node));
        } else {
            let mut current = self.start.as_mut().unwrap();
            loop {
                match current.next {
                    Some(ref mut next) => {
                        current = next;
                    },
                    None => {
                        current.next = Some(Box::new(node));
                        break;
                    },
                }
            }
        }
        index
    }
}