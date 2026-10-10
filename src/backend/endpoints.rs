pub mod Endpoints{

    pub struct UrlGetter{
        pub version: String,
    }

    impl UrlGetter{
        pub fn new(version: Option<String>) -> Self{
            Self { version: version.unwrap_or("".to_string()) }
        }
        
        pub fn normalize(&self,url: &str) -> String{
            format!("/{}/{}",self.version,url)
        }
    }


    /// Login page endpoint
    pub const LOGIN: &str = "/login";

    /// Login out endpoint
    pub const LOGOUT: &str = "/logout";

    /// Signup page endpoint
    pub const SIGNUP: &str = "/signup";
    
    /// Reset acc/alt acc endpoint
    pub const FORGOT_PASS: &str = "/reset";

    /// Refresh token endpoint
    pub const REFRESH_TOKEN: &str = "/refresh";

    /// Verify session endpoint
    pub const VERIFY_SESSION: &str = "/verify";

    /// Revoke session endpoint
    pub const REVOKE_SESSION: &str = "/revoke";

    /// Get user profile endpoint
    pub const PROFILE: &str = "/profile";

    /// Update profile endpoint
    pub const UPDATE_PROFILE: &str = "/profile/update";

    /// Change password endpoint
    pub const CHANGE_PASSWORD: &str = "/change-password";

    /// 2FA verify endpoint
    pub const VERIFY_2FA: &str = "/verify-2fa";

    /// Enable 2FA endpoint
    pub const ENABLE_2FA: &str = "/enable-2fa";

    /// Disable 2FA endpoint
    pub const DISABLE_2FA: &str = "/disable-2fa";

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