use std::fs::File;

struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File open successfully!");
    }
    else {
        println!("File open failure!");
    }

    let p = Point{x: 3, y: 6};
    println!("{}", p.x);
}