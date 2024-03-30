use std::fmt;


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String
    {
        String::from("baby")
    }
    fn baby_name_self(&self) -> String
    {
        String::from("baby")
    }
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


fn main() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()); // Error because of ambiguity as Animal doesn't have self type
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // Or to call the method with self type
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name_self(&Dog)); // baby --> This is how we can call the method with self type
    println!("A baby dog is called a {}", Animal::baby_name_self(&Dog)); 
  
    
    // now for human 
    Human::fly(&person);
    <Human as Pilot>::fly(&person);

    // Now for Wrapper
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}