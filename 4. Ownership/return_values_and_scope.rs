fn main() {
    let get_ownership = gives_ownership();
    println!("Get ownership {}", get_ownership);

    let takes_and_gives_back = takes_and_gives_back(get_ownership);
    println!("Takes and gives back {}", takes_and_gives_back);

    // get_ownership does not exist anymore because ownership has been transfer to the takes_and_gives_back fn

    let (some_string, length) = calculate_length(takes_and_gives_back);
    println!("Calculate length '{} {}'", some_string, length);
}

fn gives_ownership() -> String {   
    let some_string = String::from("yours"); 
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len(); // len() returns the length of a String

    (some_string, length)
}