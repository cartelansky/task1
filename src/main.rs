fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from(" World!");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{concatenated_string}")
}

fn concatenate_strings(x: &str, y: &str) -> String {
    let mut result = String::new();
    result.push_str(x);
    result.push_str(y);
    result
}