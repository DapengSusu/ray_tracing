
fn main() {
    // test_rev();
    test_swap();
}

#[allow(dead_code)]
fn test_rev() {
    for i in 0..10 {
        println!("{}", i);
    }
    println!("+++++++++++");
    for i in (1..10).rev() {
        println!("{}", i);
    }
}

#[allow(dead_code)]
fn test_swap() {
    let mut v:[usize; 5] = [1, 2, 3, 4, 5];

    v.swap(2, 3);

    println!("{:?}", v);
}
