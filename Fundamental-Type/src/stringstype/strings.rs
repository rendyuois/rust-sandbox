pub fn strings_type () {
    /* string literal */
    let string_literal = "\"meowww\" ini suara kucing!";
    println!("{}", string_literal);

    /* string multiple lines */
    println!("ibu pergi kepasar di malam hari.\
    beli tempe!");

    let _default_path_win = r"C:\Program Files\dcode";

    // string byte
    let c = b"GET";
    println!("string byte  {:?}", c);

    // string memory
    let noodles = "noodles".to_string();
    let oodles = &noodles[2..];
    let poodle = "ಠ_ಠ";

    println!("{} {} {}", noodles.len(), oodles, poodle.chars().count());

    let pets = vec!["vini", "vici", "veni"];
    // pets.concat(); => gabungan array
    // pets.join(",") => menggabungkan array dengan pemisah
    println!("{:?}", pets.join(","));

    let _m = "one";
    // m.to_lowercase(); => huruf kecil -> to_uppercase() -> huruf kapital
    // m.contains("o");
    // m.trim();
    
    for word in "bab, bb, bc".split(", ") {
        assert!(word.starts_with(","));
    }

    // string type alias
    type Nana = Vec<String>;

    fn decode (nama: &Nana) {
        // code
    }

}