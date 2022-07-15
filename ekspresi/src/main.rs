fn main() {

    let mn = vec!["🎈".to_string(), "🎴".to_string(), "🎮".to_string(), "john".to_string()];
    let c = &mn[3];
    valid(&c);

    for element in mn.iter() {
        if element == "🎮" {
            break;
        }
    }
    
}

fn _show (s: Vec<&str>) {
    for s in &s {
        println!("{}", *s);
    }
}

fn valid (c: &str) {
    if c == "🎈" {
        println!("Ballon!");
    } else if c == "🎴" {
        println!("Card!");
    } else if c == "🎮" {
        println!("Console!")
    } else {
        println!("❓");
    }
    println!("=> {c}")
}