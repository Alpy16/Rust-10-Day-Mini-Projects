//Goal here is to create a program that passes a large String through five different functions, in order to master
//the concept of ownership and borrowing in Rust.
fn main() {
    let mut potato = String::from("Hot Potato");

    // Passing the potato to the next function in the chain, i just named them like humans for fun
    potato = mr_uppercase(potato);
    potato = mr_lowercase(potato);
    potato = mr_snake_case(potato);
    potato = mr_kebab_case(potato);
    potato = mr_original_form(potato);

    println!("The sprint is over! Final potato: {}", potato);
}

fn mr_uppercase(mut s: String) -> String {
    s.make_ascii_uppercase();

    println!("mr uppercase has the potato and its now, {}", s);
    s
}

fn mr_lowercase(mut s: String) -> String {
    s.make_ascii_lowercase();

    println!("mr lowercase has the potato and its now, {}", s);
    s
}

fn mr_snake_case(mut s: String) -> String {
    let s = s.replace(" ", "_");
    println!("*mr_snake_case* has the potato and the result is: {}", s);
    s
}
fn mr_kebab_case(mut s: String) -> String {
    let s = s.replace("_", "-");
    println!("*mr_kebab_case* has the potato and the result is: {}", s);
    s
}

fn mr_original_form(s: String) -> String {
    let res = s.replace("-", " ");

    let mut bytes = res.into_bytes();

    if bytes.len() >= 10 {
        bytes[0].make_ascii_uppercase(); // The 'H'
        bytes[4].make_ascii_uppercase(); // The 'P'
    }

    let final_potato = String::from_utf8(bytes).unwrap();

    println!(
        "*mr_original_form* has the potato and the result is: {}",
        final_potato
    );

    // CRITICAL: You must return the value!
    final_potato
}
