mod db;
mod models;

use std::error::Error;
mod enc;
use std::io::{self, Write};

use clap::{Arg, Parser};
use db::Db;
use models::{Args, Password};

use tokio::runtime::{self, Runtime}; // Importing for user input and flush
/* 
 */
fn main()->Result<(), Box<dyn Error>>{

    
    

    // Simple connection initialization or placeholder
    let conn = runtime::Runtime::new().unwrap().block_on(Db::new()); // Use your Db::new() method to initialize the DB
    let args=Args::parse();

    if let Some(true)= args.newuser {
        // New user, ask for username and master password
        let mut username = String::new();
        let mut masterpassword = String::new();

        print!("Enter your username: ");
        io::stdout().flush()?;  // Ensure the prompt is shown immediately
        io::stdin().read_line(&mut username)?;  // Get the username input
        username = username.trim().to_string();  // Remove newline

        print!("Enter your master password: ");
        io::stdout().flush()?;  // Ensure the prompt is shown immediately
        io::stdin().read_line(&mut masterpassword)?;  // Get the master password input
        masterpassword = masterpassword.trim().to_string();  // Remove newline
        Runtime::new().unwrap().block_on(Db::insert_user(&conn.unwrap(), username,masterpassword));
    
       

        // Add any logic here to save the username and password to a database
        // For example: db.add_user(&username, &masterpassword)?;

    } else{
        match(args.name,args.masterpassword,args.add,args.delete,args.update,args.display) {
            (Some(name),Some(masterpassword),Some(password),_,_,_)=>{
                match Runtime::new().unwrap().block_on(Db::insert_password(&conn.unwrap(),name,masterpassword,password.website,password.username,password.password)){
                        Ok(_)=>{
                            println!("password added");
                        }Err(e)=>{
                            eprintln!("Error : {}",e);
                        }
                }

            },
            (Some(name),Some(masterpassword),_,Some(website,),_,_)=>{
                match Runtime::new().unwrap().block_on(Db::delete_password(&conn.unwrap(), name, masterpassword, &website)) {
                    Ok(_)=>{
                        println!("password for {} has been deleteted",website.clone());
                    }Err(e)=>{
                        println!("Error :{}",e)
                    }                    
                }
            },
           (Some(name),Some(masterpassword),_,_,Some(password),_)=>{
            match Runtime::new().unwrap().block_on(Db::update_password(&conn.unwrap(), name, masterpassword, password.website, password.username, password.password)){
                    Ok(_)=>{println!("password has been updated successfully");
                }
                    Err(e)=>{eprintln!("Error :{}",e)}
            }
           },
           (Some(name),Some(masterpassword),_,_,_,Some(_disply))=>{
             match Runtime::new().unwrap().block_on(Db::display_all(&conn.unwrap(), name,masterpassword)) {
                Ok(vecs)=>{
                    display_credentials(vecs);
                    
                }Err(e)=>println!("Error :{}",e)
                 
             }
           },
           _=>println!("Failed")

            
        }
      
    }

    Ok(()) 
}
use colored::*;
pub fn display_credentials(passwords: Vec<Password>) {
     println!("{:<15} {:<20} {}", "website", "username", "password");

    for password in passwords.iter() {
       
        println!(
            "{:<15} {:<20} {}",
            password.website.green(),           // Website in green
            password.username.blue(),                 // Username in blue
            password.password.red()                   // Password in red
        );
    }
}

