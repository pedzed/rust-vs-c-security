

fn dangling_address() -> &'static i32
{
    let a = 10;
    &a
}

fn main()
{
    let ptr: &i32;
    ptr = dangling_address();

    println!(
        "ptr = {:p} ({})",
        ptr, *ptr
    );
}
