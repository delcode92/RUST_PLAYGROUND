fn main() {
    // array into vector using `into()`
    let v:Vec<i32> = [1,2,3].into();
    let v2:Vec<String> = vec![
        String::from("data 1"),
        String::from("data2"),
        String::from("data3")
    ];

    println!("{:?}", v);
    println!("{:?}", v2[0]);
}
