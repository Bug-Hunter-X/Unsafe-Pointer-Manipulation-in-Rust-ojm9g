fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe and recommended way to modify vector elements
    println!("{:?}", v);
}