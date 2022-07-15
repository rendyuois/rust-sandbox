/* Ownership && Borrowing */

#[derive(Debug)]
struct Pixel {
    x: Vec<f64>,
    y: Vec<f64>,
}

fn main() {
    let point = vec![2.4];
    let label = vec![point, vec![3.5]];

    println!("{:?}", label);

    for label in &label {
        println!("{:?}", label);
    }

    let mut bottom = Vec::new();
    bottom.push(Pixel {x: vec![4.5], y: vec![5.5]});

    let mn = &bottom; // move

    for bottom in &bottom {
        println!("{:?} {:?}", &bottom.x, &bottom.y);
    }

    println!("{:?}", mn);

    let mut m = "john doe".to_string();
    let _n = m; // copy
    m = "doe".to_string(); 

    println!("{m} " );

    {
        // let vb = vec![1, 2, 3];
        // if c {
        //     f(vb) -> // ok move to here
        // } else {
        //     g(x) -> // ok move to here
        // }
        // e(x) // buruk tidak valid

        // while f() {
        //     g(x);
        //     x = h();
        // }
        // e(x);

        let mut vv = Vec::new();
        for i in 100..105 {
            vv.push(i.to_string());
        }

        let _v1 = std::mem::replace(&mut vv[2], "🍕".to_string());
        println!("index {:?}", vv);
    }

    {
        // koleksi vector 
        let rt = vec!["🎮".to_string(), "🎈".to_string(), "❓".to_string()];

        for mut tr in rt {
            tr.push_str(" !");
            println!("{}", tr);
        }
    }

    show_person();


}
#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: i32
}

fn show_person () {
    let id1 = Person {
        name: Some("JOhn".to_string()),
        age: 23
    };

    println!("{:?}", id1.name);
}