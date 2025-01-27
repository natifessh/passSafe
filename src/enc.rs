use std::error::Error;

use bcrypt::*;
pub fn hash_password(password:String)->Result<String,Box<dyn Error>>{
    
    let hashed_password=hash(password,DEFAULT_COST)?;
    println!("{}",hashed_password);
    Ok(hashed_password)

}
