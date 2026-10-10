pub mod Error{
    pub enum BackendErrors{
        AccountNotFound,
        InvalidBalance,
        FatalCrash(String),
        InvalidOperation(String,String), // on,why invalid
        MalformedRequest(String,String), // expected,got
        Custom(String) 
    }
}