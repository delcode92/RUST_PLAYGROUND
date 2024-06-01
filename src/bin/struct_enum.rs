#[derive(Debug)]
struct Animal{
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal{
    fn new() -> Self{
        Self{
            age: 10,
            animal_type:AnimalType::Cat   
        }
    }


    fn change_to_dog(&mut self){
        println!("change to dog");
        self.animal_type = AnimalType::Dog;
    }
    
    fn change_to_cat(&mut self){
        println!("change to cat");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&mut self){
        match self.animal_type {
            AnimalType::Cat => println!("animal is a cat"),
            AnimalType::Dog => println!("animal type is dog")
        }   
    }
    
}

fn main() {
    let mut animal = Animal::new();

    animal.change_to_dog();

    println!("animal: {:?}", animal);
    animal.check_type();
}
