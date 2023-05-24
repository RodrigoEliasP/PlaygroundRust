use std::fmt::{Display, write};

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

pub struct LinkedList<T> {
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
impl <T> LinkedList<T> 
where 
    T: ToString,
{
    fn print_node(&self, node: Option<&Node<T>>) -> String {
        if node.is_none() {
            return "".to_owned();
        }
        let current = node.expect("current node to be already validated");

        let mut str = current.data.to_string();
        str.push_str(&self.print_node(current.next.as_deref()));
        str
    }
    pub fn print_nodes(&self) -> String {
        let current = match self.start.as_deref() {
            Some(n) => Some(n),
            None => None,
        };
        self.print_node(current)
    }
}
impl<T> Display for LinkedList<T>
where 
    T: ToString, 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.print_nodes())
    }
}