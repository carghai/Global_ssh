//redis key stored in file
pub const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";
//encryption settings 
pub const ENCRYPTION: crate::db::Encrypt =  crate::db::Encrypt{
    //your encryption key
    key : "audi",
    //if you want encryption
    encryption_on : true
};
