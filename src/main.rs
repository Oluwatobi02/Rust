mod back_of_house;

fn main() {
    println!("Hello World!");
    let b1 = back_of_house::Breakfast::summer("Fries");
    println!("breakfast in the summer is {}", b1.toast)
}
