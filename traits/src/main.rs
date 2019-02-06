#![allow(dead_code)]

mod trait_objects;

fn main() {
    let mark = Person {name: String::from("Mark"), age: 20};
    let ray = Dog {name: String::from("Ray"), age: 5, breed: DogBreed::GoldenRetriver};

    println!("Info about Mark: {}", mark.info());
    println!("Info about Ray: {}", ray.info());

    trait_objects::trait_object();
}

struct Person {
    name: String,
    age: u8,
}

impl Info for Person {
    fn info(&self) -> String {
        String::from(format!("Name: {}, Age: {}", self.name, self.age))
    }
}

struct Dog {
    name: String,
    age: u8,
    breed: DogBreed 
}

impl Info for Dog {
    fn info(&self) -> String {
        String::from(format!("Name: {}, Age: {}, Breed: {}", 
                            self.name, self.age,
                            self.breed.breed_name_as_str()))
    }
}

enum DogBreed {
    GoldenRetriver,
    GermanShepherd,
    GreyHound,
}

impl DogBreed {
    fn breed_name_as_str(&self) -> &str {
        match self {
            DogBreed::GoldenRetriver => "Golden Retriver",
            DogBreed::GermanShepherd => "German Shepherd",
            DogBreed::GreyHound => "Greyhound",
        }
    }
}

//format string with info about something
trait Info {
    fn info(&self) -> String;
}