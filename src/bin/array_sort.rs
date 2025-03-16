fn main() {
    let mut arr:[i32;4] = [3,2,5,2];

    arr.sort();
    
    println!("{:?}", arr);
    println!("{:?}", arr[arr.len()-2]);
}
