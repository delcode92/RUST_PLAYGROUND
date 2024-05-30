fn main() {
    let x:i32 = 4;

    match x {
        0=>println!("match 0"),
        1=>println!("match 1"),
        2=>println!("match 2"),
        3=>println!("match 3"),
        4=>println!("match 4"),
        _=>println!("match other number")
    }


    let my_number = 6;
    let second_number:&str = match my_number{
        0=>"zero",
        5=>"five",
        6=>"six",
        _=>"other number"
    };

    println!("{}", second_number);
    

    // ======= with tuple ============
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature){
        ("cloudy", "cloud") => println!("it's dark unpleasant today"), 
        ("clear", "warm") => println!("it's a nice day"), 
        ("cloudy", "warm") => println!("it's sad but no so bad"), 
        _ => println!("not sure what the weathe is"), 
    }

    // ======== with tuple and if =============
    let children = 5;
    let married = true;

    match (children, married){
        (children, married) if married == false  => println!("not married with children {}", children), 
        (children, married) if children == 0 && married => println!("married but no children"), 
        _ => println!("married? {} number of children {}", married, children), 
    }
}
