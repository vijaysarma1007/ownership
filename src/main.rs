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
    let my_stack_value = 2;
    let my_interger_reference = &my_stack_value;
    println!("integer, {}", *my_interger_reference);

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;

    println!("reference, {my_heap_reference}");

    let number = 2;
    let copy_number = number;

    println!("copy, {number}");
    println!("second copy, {copy_number}");

    let letter = String::from("Vijay");
    let copy_letter = letter;

    // println!("letter, {letter}");
    // println!("copy letter, {copy_letter}");

    let oranges = String::from("oranges");
    print_my_value(oranges);
    //println!("oranges: {oranges}  is not valid.");

    let mut burger = String::from("burger");
    let meal = burger;
    add_fries(meal);

    let cake = bake_cake();
    println!("I now have a {cake} cake.");

    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
}

fn add_flour(mut meal: String) -> String {
    meal.push_str(" Add flour");
    meal
}

fn bake_cake() -> String {
    let cake = String::from("Chocolate mousse");
    return cake;
}

fn add_fries(mut meal: String) {
    meal.push_str(" and fries");
    println!("{meal}");
}

fn print_my_value(value: String) {
    println!("your value is: {value}");
}
