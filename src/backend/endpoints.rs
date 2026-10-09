pub mod Endpoints{
    
    /// Login page endpoint
    pub const LOGIN: &str = "/login";

    /// Signup page endpoint
    pub const SIGNUP: &str = "/signup";
    
    /// Reset acc/alt acc endpoint
    pub const FORGOT_PASS: &str = "/reset";

    /// Customer home endpoint
    pub const HOME: &str = "/home";

    /// Customer settings endpoint
    pub const SETTINGS: &str = "/settings";

    /// Sub-endpoints in settings
    pub const SETTINGS_ENDPOINTS: &[&str] = &["/name","/dob","/address","/pfp","/maritial_status","/gender","/email","/phone_no"];    

    /// Products Prefix
    pub const PRODUCTS:&[&str] = &["/product/loans","product/fixed_deposits","product/accounts","product/forex"];
    
    /// Sub-endpoints in loans product
    pub const LOANS_ENDPOINTS:&[&str] = &["/offers","/information","TnC","/custom_plan","/purchase","/track{id}","/get_all","/emi","/cancel"];

    /// Sub-endpoints in FD product
    pub const FD_ENDPOINTS:&[&str] = &["/offers","/information","TnC","/calculator","/custom_plan","/purchase","/track{id}","/get_all","/cancel"];

    /// Sub-endpoints in Account product
    pub const ACCOUNTS_ENDPOINTS:&[&str] = &["/offers","/information","TnC","/buy","/deactivate","/activate","/update_details"];

    /// Sub-endpoints in forex product
    pub const FOREX_ENDPOINTS:&[&str] = &["/offers","/information","TnC","/calculator","/purchase","/track{id}","/get_all"];

}