# passSafe
a simple cli password manager

 cargo run -- --new true this will prompt you to enter username and a master password
 cargo run -- --username urname --superpassword urmasterpassword --print all/or any word  will print all your passwords
 cargo run -- --username urname --superpassword urmasterpassword --delete "website.com" will delete your password for the website you provide
 cargo run -- --username urname --superpassword urmasterpassword --add "website.com,websiteusername,passwordforthewebite" will add a password with your @ for the website given
cargo run -- --username urname --superpassword urmasterpassword --update "website.com,websiteusername,urnewpassword" will update the password for the website with the new one

if a wrong master password is provided you will not have access to your other passwords
