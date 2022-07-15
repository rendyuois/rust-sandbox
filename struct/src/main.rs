
#[derive(Debug)]
struct GrayScale {
    x: Vec<i32>,
    y: (usize, usize)
}

fn main() {
    let widht = 10;
    let height = 2;
    let image = GrayScale {
        x: vec![0; widht * height],
        y: (widht, height)
    };

    println!("{:?}", image);
}