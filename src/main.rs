use std::io;
fn main() {
    println!("Hello, world!");
    let a = 3.7;
    let b = 5.6;
    let _c = -a / b;
    let _d : bool = true;
    let _e: char = 't';
    let f = [1, 2, 4, 5,6];
    let g :(i32, i32, i32)  = (500, 40, 2);
    let h : [i32; 5] = [3; 5];
    println!("{:?}, {:?}, {}, {}, {:?}", f, g, g.0, g.1, h);
    println!("Enter a number: ");
    let mut index = String::new();

    io::stdin()
                .read_line(& mut index)
                .expect("Failed to read line");

    let index : usize = index.trim().parse().expect("Failed to convert to a number");

    let element : i32 = h[index];
    print_result(index, element);

   
}

fn print_result(ind: usize, el: i32) {
    println!("The element at the index {ind} is {el}");
}