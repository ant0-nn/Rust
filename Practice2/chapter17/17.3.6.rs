struct Interface<'a> {
    manager: &'a mut Manager,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager {
    text: String,
}

struct List {
    manager: Manager,
}

impl List {
    pub fn get_interface(&mut self) -> Interface<'_> {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: String::from("hello") },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    process_list(&list);
}

fn process_list(list: &List) {
    println!("{}", list.manager.text);
}