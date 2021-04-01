mod funct;

use std::io::{stdin, stdout, Write};
use rpassword::read_password_from_tty;

fn main() {

    // First, prompt the user for a login and a password
    let mut username = String::new();
    print!("Username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap();
    username.pop(); // remove the trailing '\n'
    let password: String = read_password_from_tty(Some("Password: ")).unwrap();


    //Call the simple function
    let status = funct::simplepam(&username, &password);
    println!("{}",&status);


    //Call the complex function
    funct::complexpam(&username, &password);
    
}