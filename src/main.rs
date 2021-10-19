use std::io::stdin;

#[derive(Debug)]
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
    let mut visitor_list = vec![
        Visitor::new("rance", "Hello supreme leader"),
        Visitor::new("maddy", "Hi sexy woman"),
        Visitor::new("mason", "Howdy Partner"),
        Visitor::new("marcus", "So cute baby"),
    ];

    loop {
        println!("Hello, what's your name? (ENTER to quit)");
        let your_name = what_is_your_name();
        println!("You said your name is, {}?", &your_name);
        let known_visitor = visitor_list.iter().find(|v| v.name == your_name);
        match known_visitor {
            Some(v) => v.greet_visitor(),
            None => {
                if your_name.is_empty() {
                    break;
                } else {
                    println!(
                        "Sorry {}, I'll add you to the list for the next party.",
                        your_name
                    );
                    visitor_list.push(Visitor::new(&your_name, "What's up bruh"));
                }
            }
        }
    }

    println!("The final list of visitors: ");
    println!("{:#?}", visitor_list);
}
