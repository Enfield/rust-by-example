fn main() {
    //auto replacement
    println!("{} days", 31);

    //as positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //as named arguments
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    //special formatting
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    //right align
    println!("{number:>width$}", number = 1, width = 6);

    //numbers with extra zeros
    println!("My name is {0}, {1} {0}", "Bond", "James");

    //structure with i32
    let pi = 3.141592;
    println!("The Pi number is {0:.3}", pi);
}