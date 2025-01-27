use std::str::FromStr;

use clap::Parser;
#[derive(Debug,Clone)]
#[derive(sqlx::FromRow)]
pub struct Password{
    pub  website:String, 
    pub username:String,
    pub password:String
}
impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err("Invalid password format, expected 'website,username,password'".to_string());
        }
        Ok(Password {
            website: parts[0].to_string(),
            username: parts[1].to_string(),
            password: parts[2].to_string(),
        })
    }
}
pub struct User{
    pub username:String,
    pub masterpasword:String
}
#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
pub struct Args{
    #[arg(short='n',long="new")]
    pub newuser:Option<bool>,
    #[arg(short='u',long="username")]
    pub name:Option<String>,
    #[arg(short='s',long="superpassword")]
    pub masterpassword:Option<String>,
    #[arg(short='a',long="add")]
    pub add:Option<Password>,
    #[arg(short='m',long="update")]
    pub update:Option<Password>,
    #[arg(short='P',long="print")]
    pub display:Option<String>,
    #[arg(short='d',long="delete")]
    pub delete:Option<String>,/* 
    #[arg(short='g',long="generate")]
    pub generate:Option<String> */
  
    
        
    

}