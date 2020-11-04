


#[allow(unused_assignments)]
fn main()
{
    let mut hello = String::new();

    hello = String::from("Hello!");
    println!("hello={}", hello);

    // NOTE: It is not possible to free memory
    // manually within the safe Rust environment

    println!("hello={}", hello);
}
