// const should always have type
const DIVIDER: &str = "---------------------------------------------------";

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

    // Examples: setting types explicitly
    let name_explicit: &str = "Alice";
    print!("Name is set as {} \n", name_explicit);
    let name_explicit2: &str;
    name_explicit2 = "Suse";
    print!("Name is set as {} \n", name_explicit2);
}

fn main() {
    println!("Hello Rust!\n{}", DIVIDER);
    println!("Using variables\n{}", DIVIDER);
    variables();
}
