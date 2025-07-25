#![allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_mut)]
use std::io;
//I like Dead Code for Debugging
//This is Calculator App Made By Devanshu
fn main() {
    //main Function will call the start later
    //for debugging
    greet();
}
fn greet(){
    let msg = "Hello There this is CalCulator created.";
    let nameQues = "Please Enter your name";
    println!("{}\n{}",msg , nameQues);
    let name: String = input();
    println!("Hello {}", name);
    listOptions(name);
}
fn listOptions(name: String) {
    println!("What Operation Would you like to perform:");
    println!("1 => Addition");
    // println!("2 => Subtraction");
    // println!("3 => Multiplication");
    // println!("4 => Division")
    let mut choice:i8 = inputChoice();
    executeChoice(choice);
}
fn executeChoice(choice: i8) {
    match choice {
        1 => addition(),
        _ => greet(),
    }
}
fn addition() {
    println!("Please Enter Initial Number to add:");
    let mut x:i128 = input().trim().parse().expect("Error, Something that should be a number is a not a number");
    println!("Please Enter Other Number:");
    let mut y:i128 = input().trim().parse().expect("Error, Something that should be a number is a not a number");
    let z = x +y;
    println!("The Output is: {}" ,z)
}
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
// A function is needed for input of numbers
fn inputChoice() -> i8 {
    let mut choice:i8 = 0;
    let tempChoice = input();
    choice = tempChoice.trim().parse()
    .expect("You didnt Enter a Number , Terminating Program");
    return choice;
}