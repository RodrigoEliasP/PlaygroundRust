use std::{fmt::{Display, write}, error::Error, ops::DerefMut};

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
    config: LinkedListConfig,
}

pub struct LinkedListConfig {
    printing_separator: char,
}

impl<'i, T> LinkedList<T> {
    pub const fn new<'a>() -> Self {
        let default_config = LinkedListConfig {
            printing_separator: ';'
        };
        
        LinkedList { start: None, qtd: 0, config: default_config }
    }
    fn new_node(&mut self,data: T) -> Node<T> {
        Node {
            next: None,
            data
        }
    }

    pub fn get_size(&self) -> u32 {
        self.qtd
    }

    pub fn change_config(&mut self, config: LinkedListConfig) {
        self.config = config;
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
    pub fn find<'a>(&'a self, comparator: fn(&T) -> bool) -> Result<&'a T, String> {
        let mut current = self.start.as_deref();
        
        loop {
            match current {
                Some(node) => {
                    let value = &node.data;
                    if comparator(value) {
                        return Ok(value);
                    };
                    current = node.next.as_deref();
                },
                None => {
                    return Err("Not found".to_owned())
                },
            }
        }
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
        str.push_str(";");
        str.push_str(&self.print_node(current.next.as_deref()));
        str
    }
    pub fn print_nodes(&self) -> String {
        let current = match self.start.as_deref() {
            Some(n) => Some(n),
            None => None,
        };
        let str = self.print_node(current);
        str.strip_suffix(
            self.config.printing_separator
        ).expect("string without suffix").to_owned()
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