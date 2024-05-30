fn main() {
    let country:String = String::from("ireland");
    let ref_one = &country;
    let ref_two = &country;
   
    let n1:i32 = 20;
    let n2 = &n1;
    let n3 = &n1;

    println!("{}", country);

    let x = 5;
    let y = x;
    let z = x;

    println!("{}", y);
    println!("{}", y);
    println!("{}", z);


    // ========= VECTOR SAMPLE ==========
    let v_sample:Vec<i32> = vec![0,20,13];
    println!("{:?}", v_sample);
    let v2 = v_sample;

    // we could not print v_sample again because it not simple scalar type (i32, i64, f32 ... etc)
    //println!("{:?}", v_sample);
}   
