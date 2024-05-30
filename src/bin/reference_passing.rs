//fn show_country()->&'static String{

//fn show_country()->&'static str{
//fn show_country<'a>()->&'a str{

fn show_country<'a>()->&'a String{
    let x:String = String::from("indonesia");
    //let y:&String = &x;
    &x
    
    //let x2:&str = "german";
    //x2
}


fn main() {
       let z = show_country();
        println!("{}", z);
}
