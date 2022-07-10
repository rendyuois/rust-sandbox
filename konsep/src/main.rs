/* konsep bahasa pemrograman rust-lang */



#[warn(dead_code)]
fn main () {
    /* Vari&&&able dan Mutability */
    
    let mut x = vec![]; // variable mutability bisa di ubah kepemilikan nya
    x.push(10); // return mutability 
    
    //println!("Value x : {:?}", &x); // value x valid
    
    let y = String::from("Hi, Mom!");
    
    let _yx = &y; // value y move ke value yx {let ? = &?}
    //println!("Value yx {}\nValue y : {}", yx, &y); 
    
    
    
    /* Data Type
    int -> i8/u8 i32/u32 i64/u64 i128/u128 isize/usize
    float -> f32 & f64
    chart -> chart
    boolean -> bool
    tuple -> (i32, i32) = (i32, i32)
    array -> 1234 & vec![]
    */
    
    let _dt = 100;
    // const Price: i32 = 29_000;
    let _dtt:Vec<u8> = vec![10];
    let _some_string = "Mom!";
    let _cr = "🎮";
    let _valid = true;

    let _tp = (23, 24);
    let _ds = _tp.0; // akses indeks tuple {?.index}

    let _ar = vec!["Salad", "Jeruk", "John", "Doe"]; // array vector
    let _len = _ar[1]; // akses indeks array {?[index]}

    println!("{:?}", _len);
    
    /* Function */
    some_string("John Doe".to_string());
    kalkulator(10, 5);


    /* control flow */

    {
        let mut count = 0;

        let result = loop {
            count += 1;

            if count == 10 {
                break count * 2;
            }
        };
        println!("{result}");
    }

    let a = vec![12, 34, 56, 76, 45];
    let mut _index = 1;

    for mut _index in 0..5 {
        println!("index : {}", _index);

    _index += 1;
    }

    while _index < 5 {
        println!("value : {}", a[_index]);

        _index += 1;
    }

}

fn some_string (name: String) {
    let _len = name.len();
    println!("{}", name);
}

fn kalkulator (count1: i32, count2: i32) {
    let kali = count1 * count2;
    println!("{}", kali);
}

