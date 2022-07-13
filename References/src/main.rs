/* References */

use std::collections::HashMap;


type Table = HashMap<String, Vec<String>>;

fn show (l: Table) {
    for (name, job) in l {
        println!("Name : {}", name);
        for jobs in job {
            println!("Job : {}", jobs);
        }
    }
}
fn main() {
    // reference to value
    let mut list = Table::new();
    list.insert("John".to_string(), vec!["Penjahit".to_string()]);

    show(list);
}