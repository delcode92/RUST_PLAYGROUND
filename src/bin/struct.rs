fn main() {
    #[derive(Debug)]
    struct Animal {
        animal_type: String,
        foot_count: i32,   
    }
    
    let dog = Animal{ animal_type:String::from("Dog"), foot_count:4 };

    println!("{:?}", dog);

    println!("{:?}", dog.foot_count);
    println!("{:?}", dog.animal_type);

}
