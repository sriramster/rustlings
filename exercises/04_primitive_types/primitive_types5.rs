fn main() {
    let cat: (String, f32) = ("Furry McFurson".to_string(), 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age) = cat;
    
   println!("{name} is {age} years old");
}
