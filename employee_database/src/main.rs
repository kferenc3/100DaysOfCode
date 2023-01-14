use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

//going forward the project names as the given day are not the best idea as this took two day for me (given the available time) so this is day 13 and 14
//a simple script to manipulate hashmaps. Follow the instructions! It doesn't handle a looot of scenarios, but given my current skill I'm quite satisfied with the result.

fn main() {
    println!("\nWelcome to the EmployeeAdderUltimate200\n");
    let mut emp_db: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut opt = String::new();
        
        println!(
            "\nEnter the number of the desired activity:\n
            1) Add new employee
            2) Remove an employee
            3) List employees alphabetically
            4) List employees by department
            5) Quit"
        );

        io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read input. Please enter an option between 1 and 5!");
        
        let opt: i8 = match opt.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number.\n\n");
                continue;
        },
        };
        
        match opt {
            1 => {
                let mut inp: String = String::new();
                println!("To add an employee type: Add <name> to <department>\n");
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Not a valid input.");
                
                let k = inp.split_whitespace().nth(3).unwrap().to_string();
                match emp_db.entry(k) {
                    Entry::Vacant(e) => {e.insert(vec![inp.split_whitespace().nth(1).unwrap().to_string()]);},
                    Entry::Occupied(mut e) => {e.get_mut().push(inp.split_whitespace().nth(1).unwrap().to_string())},
                }
                println!("{:?} added to {:?}",inp.split_whitespace().nth(1).unwrap(),inp.split_whitespace().nth(3).unwrap());
                continue;
            },
            2 => {
                let mut inp: String = String::new();
                println!("To delete an employee type: Delete <name> from <department>\n");
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Not a valid input.");

                let dep = inp.split_whitespace().nth(3).unwrap().to_string();
                let emp = inp.split_whitespace().nth(1).unwrap().to_string();
               
                if emp_db.contains_key(&dep) {
                    let v = emp_db.get_mut(&dep).unwrap();
                    if let Some(pos) = v.iter().position(|x| *x == emp) {
                        v.swap_remove(pos);}
                } else {
                    println!("No such department found.");
                }
                println!("{} has been removed from {}", emp, dep);
                continue;
            },
            3 => {
                println!("This will list employees alphabetically!\n\n");
                let mut sorted_names: Vec<&String> = Vec::new();
                for val in emp_db.values() {
                    for names in val {
                        sorted_names.push(names);
                    }
                }
                sorted_names.sort();
                println!("{:?}",sorted_names);
                continue;
            },
            4 => {
                println!("This will list employees by department!\n\n");
                println!("{:?}", emp_db);
                continue;
            },
            5 => {
                println!("Bye!\n\n");
                break;
            },
            _ => {
                println!("That is not an available option. Please enter a number between 1 and 5.\n\n");
                continue;
            }

        };
    };
}