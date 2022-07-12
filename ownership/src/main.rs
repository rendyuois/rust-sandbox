/* Ownership And Borrowing */

fn main () {

    let s = "John";

    {
        let m = &s;
        let mm = &m[2..4];

        println!("{mm}");
    }

    println!("{s}");
}