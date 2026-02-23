fn main() {
    let food: &str = "pasta";

    let text = String::new();
    let candy = String::from("KitKat");

    let mut name = String::from("Boris");
    println!("{name}");
    name.push_str(" Pask");

    println!("{name}");


    //clone
    let person = String::from("Boris");
    let genius = person.clone();

    println!("person, {person}");
    println!("genius, {genius}");

    //borrowing
}
