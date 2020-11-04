


fn main()
{
    let ptr: Box<i32>;
    ptr = Box::new(123);
    println!("ptr = {:p} ({})", ptr, *ptr);

    let ptr2: Box<char>;
    ptr2 = Box::new('A');
    println!("ptr2 = {:p} ({})", ptr2, *ptr2);
}
