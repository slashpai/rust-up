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

fn operators() {
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);
}

fn conditionals() {
    // if
    let x = 10;
    if x % 2 == 0 {
        println!("{} is a even number", x);
    }

    // if else
    let age = 16;
    if age >= 18 {
        println!("You are above 18, You can vote.");
    } else {
        println!("You are below 18, too young to vote.");
    }

    // if else if
    let score = 85;

    if score >= 90 {
        println!("Your score is {}, Grade: A", score);
    } else if score >= 80 {
        println!("Your score is {}, Grade: B", score);
    } else if score >= 70 {
        println!("Your score is {}, Grade: C", score);
    } else {
        println!("Your score is {}, Grade: F", score);
    }

    // using if as an expression
    // When using if as an expression, you must include else. This ensures the result always has a value.
    let time = 20;
    let greeting = if time < 12 {
        "Good Morning."
    } else {
        "Good evening."
    };
    println!("{}", greeting);
}

fn main() {
    println!("Hello Rust!");
    println!("{}\nString Formatting Placeholders\n{}", DIVIDER, DIVIDER);
    string_formatting_placeholders();
    println!("{}\nUsing variables\n{}", DIVIDER, DIVIDER);
    variables();
    println!("{}\nChanging variables\n{}", DIVIDER, DIVIDER);
    change_variables();
    println!("{}\nOperators\n{}", DIVIDER, DIVIDER);
    operators();
    println!("{}\nConditionals\n{}", DIVIDER, DIVIDER);
    conditionals();
}
