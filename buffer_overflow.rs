

fn main()
{
    let mut array_elements: [i32; 3] = [0; 3];

    array_elements[0] = 123;
    array_elements[1] = 234;
    array_elements[2] = 345;
    array_elements[3] = 456;

    println!(
        "array_elements[3] = {}",
        array_elements[3]
    );
}
