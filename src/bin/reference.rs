fn main() {
    let country:String = String::from("ireland");
    let ref_one = &country;
    let ref_two = &country;
   
    let n1:i32 = 20;
    let n2 = &n1;
    let n3 = &n1;

    println!("{}", country);
}   
