use std::io;
//This is Calculator App Made By Devanshu
fn main() {
    //main Function will call the start later
    //for debugging
    let name = input();
    print!("{}",name)
}
// fn greet(){
//     let msg = "Hello There this is CalCulator created.";
//     let nameQues = "Please Enter your name";
// }
// fn listOptions() {
    
// }
//I need to Create a Input funtion to take input
fn input() -> String{
    //creating a empty mutable string
    let mut inp: String = String::new();
    //getting string from module
    io::stdin()
        .read_line(&mut inp)
        .expect("A fatal error has occured");
    return inp;
}