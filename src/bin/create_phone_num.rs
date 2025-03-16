fn extract(arr:&[i32] ){
    for &n in arr{
        println!("{}", n);
    }
}

fn main(){
    let array_num:[i32;10] = [1,2,3,4,5,6,7,8,9,0];

    let first_sec:&[i32] = &array_num[0..3];
    // let second_sec:&[i32] = &array_num[3..6];
    // let third_sec:&[i32] = &array_num[6..10];

    extract(first_sec);

    // println!("{:?}", first_sec);
    // println!("{:?}", second_sec);
    // println!("{:?}", third_sec);

    // for i in first_sec.iter(){
    //     println!("Hello {}", i);
    //     // match i {
    //     //     // -1 => println!("There is a rustacean among us!"),
    //     //     _ => println!("Hello {}", i),
    //     // }
    // }

}