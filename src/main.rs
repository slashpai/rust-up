// const should always have type
const DIVIDER: &str = "---------------------------------------------------";

fn string_formatting_placeholders() {
    println!("Hello, {}!", "world");
    println!("{name}", name = "Alice");
}

fn variables() {
    // Examples: type of a variable is decided by the value you give it.
    let literal = 'C';
    println!("You have set literal as {}", literal);
    let name = "Bob";
    println!("You have set name as {}", name);
    let num = 10;
    println!("You have set integer number as {}", num);
    let floatnum = 10.5;
    println!("You have set decimal number as {}", floatnum);
    let boolean = true;
    println!("You have set boolean number as {}", boolean);

    // Examples: setting types explicitly
    let name_explicit: &str = "Alice";
    print!("Name is set as {} \n", name_explicit);
    let name_explicit2: &str;
    name_explicit2 = "Suse";
    print!("Name is set as {} \n", name_explicit2);

    let my_num: i32 = 5; // integer
    let my_float: f64 = 5.99; // float
    let my_letter: char = 'D'; // character
    let my_bool: bool = true; // boolean
    let my_text: &str = "Hello"; // string
    println!(
        "Values set with explicit types: {} {} {} {} {}",
        my_num, my_float, my_letter, my_bool, my_text
    );
}

fn change_variables() {
    let mut x = 5;
    println!("Before changing value: {}", x);
    x = 10;
    println!("After changing value: {}", x);
}

fn main() {
    println!("Hello Rust!");
    println!("{}\nString Formatting Placeholders\n{}", DIVIDER, DIVIDER);
    string_formatting_placeholders();
    println!("{}\nUsing variables\n{}", DIVIDER, DIVIDER);
    variables();
    println!("{}\nChanging variables\n{}", DIVIDER, DIVIDER);
    change_variables();
}
