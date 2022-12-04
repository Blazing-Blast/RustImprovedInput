mod lib;

use lib::{readline, readlines};

#[allow(dead_code)]
enum Gender {
    MALE,
    FEMALE,
    OTHER
}

#[allow(dead_code)]
struct Person {
    gender: Gender,
    first_name:String,
    last_name: String,
    age: u8,
    bio: String
}

fn main() {
    let myself:Person = Person {
        gender: Gender::MALE,
        first_name: readline(12, "What is your first name (max 12 characters)? "),
        last_name:  readline(12, "What is your last name (max 12 characters)? "),
        age: 123,
        bio: readlines(32, "What do you want your bio to be (exactly 32 charctes)? ")};
    println!("{}", myself.call()); // >> BLAZING BLAST, GET OVER HERE!
    println!("Also your bio is: {}", myself.bio);
}

fn shout(s: &mut String) -> &mut String{
    s.make_ascii_uppercase();
    *s = s.replace(".", "!");
    s
}

impl Person {
    fn call(&self) -> String{
        let mut out:String = String::from(&self.first_name);
        out += " ";
        out += &self.last_name;
        out += ", get over here.";
        shout(&mut out);
        out
    }
}