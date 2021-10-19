use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    age: i8,
    action: VisitorAction,
}

impl Visitor {
    fn new(name: &str, age: i8, action: VisitorAction) -> Self {
        Self {
            name: name.to_lowercase(),
            age,
            action,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome tot he tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Get out of here fart sniffer"),
        }
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
        Visitor::new("rance", 27, VisitorAction::Accept),
        Visitor::new("maddy", 25, VisitorAction::Accept),
        Visitor::new(
            "mason",
            2,
            VisitorAction::AcceptWithNote {
                note: String::from("Watch out for this tornado"),
            },
        ),
        Visitor::new("marcus", 0, VisitorAction::Accept),
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
                    visitor_list.push(Visitor::new(&your_name, 39, VisitorAction::Accept));
                }
            }
        }
    }

    println!("The final list of visitors: ");
    println!("{:#?}", visitor_list);
}
