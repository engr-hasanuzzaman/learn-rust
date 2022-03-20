pub fn run() {
    let mut a: Vec<i32> = vec![1,2,3,4,5];
    println!("Full array is {:?}", a);
    println!("First array element is {:?}", a[0]);

    a[4] = 120;
    println!("After chanig the full array is {:?}", a);
    a.push(100);
    println!("After adding new value the array is {:?}", a);
    println!("array size is {:?}", a.len());
    println!("array memo size is {:?}",std::mem::size_of_val(&a));
    println!("array slicing is {:?}", &a[0..2]);
}