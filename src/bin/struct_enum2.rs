#[derive(Debug)]
enum Mood {
    Good,
    Bad,
    Sleepy
}

impl Mood {
    fn check(self){
        match self {
            Self::Good => println!("I'm feeling good"),
            _ => println!("...")
        }
   }
}

fn main() {
    let my_mood = Mood::Good;

    my_mood.check();
}
