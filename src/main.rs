mod db;
mod models;

use std::error::Error;
mod enc;
use std::io::{self, Write};

use clap::{Arg, Parser};
use db::Db;
use models::{Args, Password};

use tokio::runtime::{self, Runtime}; 
fn main()->Result<(), Box<dyn Error>>{
    let conn = runtime::Runtime::new().unwrap().block_on(Db::new());
    let args=Args::parse();

    if let Some(true)= args.newuser {
      
        let mut username = String::new();
        let mut masterpassword = String::new();

        print!("Enter your username: ");
        io::stdout().flush()?;  
        io::stdin().read_line(&mut username)?;  
        username = username.trim().to_string(); 

        print!("Enter your master password: ");
        io::stdout().flush()?;  
        io::stdin().read_line(&mut masterpassword)?;  
        masterpassword = masterpassword.trim().to_string(); 
        Runtime::new().unwrap().block_on(Db::insert_user(&conn.unwrap(), username,masterpassword));
    
       

        

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
            password.website.green(),           
            password.username.blue(),                 
            password.password.red()                   
        );
    }
}

