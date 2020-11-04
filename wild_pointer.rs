

fn main()
{
    let ptr: &i32;
    println!(
        "ptr={:p} ({})",
        ptr, *ptr
    );

    let ptr2: &char;
    println!(
        "ptr2={:p} ({})",
        ptr2, *ptr2
    );
}
