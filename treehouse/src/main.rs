use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("kinu", "Konnichiwa Kinu chan, How's it going?"),
        Visitor::new("mugi", "Otsukare Mugi kun, I like your drawing."),
        Visitor::new("herbert", "I enjoy reading this book, thanks."),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let known_visitor = visitor_list
        .iter() // create an iterator that contains all of the data from the visitor_list
        .find(|visitor| visitor.name == name); // find() runs a closure
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
}
