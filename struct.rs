

fn greet(){
    println!("Good morning!");
}

fn main(){


    struct Person {
        name: String,
        age: u8,
        height: u8,
    };

    let person = Person {
        name: String::from("Iyappan"),
        age: 20,
        height: 170,
    };

    println!("Name = {}", person.name);

    greet();

}