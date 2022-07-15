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

fn sort_works (t: &mut Table) {
    for (_name, job) in t {
        job.sort();
    }
}
fn main() {
    // reference to value
    let mut list = Table::new();
    list.insert("John".to_string(), vec!["Penjahit".to_string()]);

    let mut person = Table::new();
    person.insert("Doe".to_string(), vec!["Pedagang".to_string()]);

    show(person);
    //sort_works(&mut list);

    {
        let mut m = 0;
        let n = &mut m; // referensi yang bisa merubah nilai y
        loop {
            if n > &mut 10 {
                break *n *= 2;
            }
            *n += 1;
        }
        println!("value n = {n}");

    }

    // Borrowing local variable
    let ex;
    {
        let x = 12;
        ex = &x;

        println!("value ex = {}", *ex);
    }

    {
        // reference to reference
        let r = Point {
            _x: 100,
            _y: 200
        };

        let _r1 = &r;
        let _r2 = &_r1;
        let _r3 = &_r2;

        println!("{:?}", _r3);
    }

    {
        // reference sebagai argumen fungsi
        static mut STCH: &i32 = &233;
        fn foo(p: &'static i32) {
           unsafe {
            STCH = p;
           }
        }

        static FEED: i32 = 1000;
        foo(&FEED);

    }

    {
        // meneruskan referece ke fungsi
        fn smallest<'a> (v: &[i32]) -> &i32 {
            let mut s = &v[0];
            for r in &v[1..] {
                if *r < *s { s = r; };
            }
            s
        }

        let po = vec![1, 2, 3, 4, 2, 1, 2, 3];
        let s = smallest(&po);
        println!("value smalles => {:?}", *s);
    }

    {
        // struct yang mengandung reference
        /* 

        struct S {
            ex: &'static i32 => mendifinisikan referensi ke definisi tipe lain
        }

        struct S<&'a> {
            ex: &'a i32 => parameter seumur hidup a digunakan untuk r
        }

        struct D<'a> {
            ex: S<'a> => memberikan parameter seumur hidupnya D dan meneruskannya ke S
        }
        
        */
    }

    {
        /*  parameter masa hidup berbeda

        struct D<'a> {
            a: 'a i32,
            b: 'a i32
        }
        
        struct D<'a, 'b> { => referebce memiliki masa hidup berbeda
            ex: 'a i32,
            ex: 'b i32
        }

        fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 { r } // perhaps too tight
        fn f<'a, 'b>(a: &'a i32, b: &'b i32) -> &'a i32 {a} // looser


        */

        let ex = StringTable {
            elements: vec!["dfdsdf".to_string()]
        };

    }
}

#[derive(Debug)]
struct Point {
    _x: i32,
    _y: i32
}

#[derive(Debug)]
struct StringTable {
    elements: Vec<String>
}
impl StringTable {
    fn find_by_prefix<'a, 'b> (&'a self, prefix: &'b str) -> Option<&'a String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            };
        }
        None
    }
}
