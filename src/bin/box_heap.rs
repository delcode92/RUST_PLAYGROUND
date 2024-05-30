fn get_greeting(name: &str) -> &String {
  let greeting = format!("Hello, {}!", name);
  let greeting_boxed = Box::new(greeting); // Allocate memory on the heap
  &*greeting_boxed // Return a reference to the allocated string
}

fn main() {
  let name = "Alice";
  let greeting = get_greeting(name);
  println!("{}", greeting); // Prints "Hello, Alice!"
}

