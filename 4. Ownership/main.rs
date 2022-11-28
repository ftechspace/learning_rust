fn main() {
    // let s = "Hello";
    let mut t = String::from("Hello"); 
    let _x = String::from(""); 
    if s == "Hello" {
        let u = String::from("Hello"); 
        t = u;
        t.push_str(", World!");
    }
    


    clone_data();
    takes_ownership(t);
    // println!("afterward T: {}", t);   //calling t after ownership has been taken

    let y = 5;
    make_copy(y);
    println!("afterward Y: {}", y);   //calling t after making a copy

    // Return Values and Scope

}
// fn to clone a data 
fn clone_data(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("Clone dat: {} {}", s1, s2);
}
// This fn takes ownership 
fn takes_ownership(some_string: String){
    println!("Take ownership: {}", some_string);
}

// This fn makes copy
fn make_copy(some_string: i32){
    println!("make copy {}", some_string)
}
