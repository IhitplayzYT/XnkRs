pub mod Entities{
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate,ValidationError};

    #[derive(Debug,Serialize,Deserialize,Validate)]
    pub struct User{
        id: Uuid,
        #[validate(email)]
        email: String,
        #[validate(custom(function="validate_phone_no_v2"))]
        phone: String,
    }

    impl User{
        /// Used to create a new User
        /// Args:
        ///     email: String = The email of user to be created
        ///     phone: String = The phone_no inclusive of countrycode,for user to be created
        ///     is_customer: bool = Uuid generation changes based on if user is customer or not
        pub fn new(email:String,phone: String,is_customer: bool) -> Self{
            // Based on if user is customer we allot uuid
            // v4 is completely random
            // v7 is timestamp based prefix + random 
            // We add timestamp to temp users
            Self { id: if !is_customer {Uuid::now_v7()} else{Uuid::new_v4()} , email, phone }
        }

        // Id Getter
        pub fn get_id(&self) -> Uuid{ self.id}
        // Id Setter
        pub fn set_id(&mut self,id: Uuid){ self.id = id; }
        // Regenrate Id 
        pub fn new_id(&mut self,is_customer: bool){ self.id = if !is_customer {Uuid::now_v7()} else{Uuid::new_v4()};}

        // Email Getter
        pub fn get_email(&self) -> String {self.email.clone()}
        // Email Setter
        pub fn set_email(&mut self,email:String) {self.email = email;}

        // Phone getter
        pub fn get_phone_no(&self) -> String {self.phone.clone()}
        // Phone Setter
        pub fn set_phone_no(&mut self,phone:String) {self.phone = phone;}
    }


    /// Audit Fields for Structures
    #[derive(Debug,Deserialize,Serialize)]
    pub struct Audit{
        pub uid: Uuid,
        pub uuid: Uuid,
        pub ldbid: Uuid,
        pub ws_id: Uuid,
        pub effective_date: DateTime<Utc>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub host_timestamp: NaiveDateTime,
    }

    impl Audit{
        /// Create new Audit struct
        pub fn new(uid: Uuid,ldbid: Uuid,ws_id:Uuid) -> Self{
            Self { uid,uuid: Uuid::now_v7(), ldbid , ws_id, effective_date: Utc::now(), created_at: Utc::now(), updated_at: Utc::now(), host_timestamp: Utc::now().naive_local()}
        }
    }

    #[derive(Debug,Clone, Copy,Serialize,Deserialize)]
    pub enum Role{
        Admin,
        User,
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct UserMetaData{
        pub role: Role,
        pub is_valid: bool,
        pub user_meta_data_id: Uuid
    }

    impl Default for UserMetaData{
        fn default() -> Self {
            Self { role: Role::User, is_valid: true,user_meta_data_id: Uuid::new_v4()}
        }
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct Customer{
        user: User,
        user_metadata: UserMetaData,
        audit_fields: Audit,
        customer_type: Customer_t,
        balance: AccountBalance,
        relationships: Vec<Relationship>
    }

    impl Customer{
        /// Used to create a new Customer 
        /// Args:
        ///     email: String = The email of user to be created
        ///     phone: String = The phone_no inclusive of countrycode,for user to be created
        ///     is_customer: bool = Uuid generation changes based on if user is customer or not
        ///     ldbid: Option<Uuid> = Uuid of logical database where customer to be storred else a new one is generated
        ///     ws_id: Option<Uuid> = Uuid of workspace from where customer creation request is sent from else a new one is generated
        pub fn new(email:String,phone: String,is_customer: bool,ldbid: Option<Uuid>,ws_id: Option<Uuid>,money: Money) -> Self{
            let user = User::new(email, phone, is_customer);
            let (uuid,ldbid,ws_id) = (user.get_id(),ldbid.unwrap_or(Uuid::new_v4()),ws_id.unwrap_or(Uuid::new_v4()));
            let mut balance = AccountBalance::new(Some(money.currency));
            balance.available = money;
            Self { user,user_metadata: UserMetaData::default() , audit_fields:Audit::new(uuid, ldbid, ws_id),customer_type: Customer_t::Normal,relationships:vec![],balance}   
        }

        /// Create new user from the individual components if present
        pub fn new_from(user: User,user_metadata: UserMetaData,audit_fields: Audit,customer_type: Customer_t,balance:AccountBalance) -> Self{
            Self { user, user_metadata, audit_fields,customer_type,relationships:vec![],balance}
        }
    }



    /// Validate international phone number standards
    /// v2 api used Internally to validate phone numbers
    pub fn validate_phone_no_v2(phone: &str) -> Result<(), ValidationError> {
        match phonenumber::parse(None, phone) {
            Ok(number) if phonenumber::is_valid(&number) => Ok(()),
            _ => Err(ValidationError::new("invalid_phone_number")),
        }
    }



    #[deprecated]
    /// Internal validation function to validate phone numbers
    /// v1 supporting simple logic
    pub fn validate_phone_no_v1(phone: &str) -> Result<(),ValidationError>{
        let phone = if phone.starts_with("+") {phone.strip_prefix("+").unwrap()}else{phone};
        if let Some(z) = phone.find("-"){
            let (country_code,phone_no) = phone.split_at(z);
            if phone_no.len() != 10{
                return Err(ValidationError::new("Phone number has to be ten digits long"));
            }
            if !phone_no.as_bytes().iter().all(|&x| x >= b'0' && x <= b'9'){
                return Err(ValidationError::new("Phone number is a numeric only"));
            }
            let numeric_country_code = country_code.parse::<u16>().map_err(|_| {
                ValidationError::new("Country code is a numeric only data")
            })?;

            if numeric_country_code < 1 || numeric_country_code > 999{
               return Err(ValidationError::new("Country code lies between 1 to 999 only"));
            }
        }else{
            return Err(ValidationError::new("No hiphen seperating country code and number is detected!"))
        }
        Ok(())
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct Relationship{
        pub action_id: Uuid,
        pub relate: Relate,
    }

    impl Relationship{
        // Make a new Relationship
        pub fn new(relate: Relate) -> Self{
            Self { action_id: Uuid::now_v7(), relate}
        }
    }

    #[derive(Debug,Serialize,Deserialize,Clone, Copy)]
    pub enum Relate{
        Debtor(Uuid,Uuid),
        Creditor(Uuid,Uuid),
        Trustee(Uuid,Uuid),
        Beneficiary(Uuid,Uuid),
        Bailor(Uuid,Uuid),
        Bailey(Uuid,Uuid),
        Signator(Uuid,Uuid),
        Nominee(Uuid,Uuid),
        Joint(Uuid,Uuid)        
    }


    pub struct Product{
        pub product_category_id: Uuid,
        pub product_id: Uuid,
        pub name: String,
        pub desc: String,
        pub product_type: Product_t,
    }

    impl Product{
        /// Created a new product
        /// Args:
        ///     product_category_id: Uuid  = The general topmost category of the product
        ///     name: String = Name of the product 
        ///     desc: String = Description of the product 
        ///     product_type: Product_t = Type of product
        pub fn new(product_category_id: Uuid,name: String,desc:String,product_type: Product_t) -> Self{
            Self { product_category_id, product_id: Uuid::new_v4(), name, desc, product_type }
        }

    }


    pub enum Product_t{
        Account(Account),
        Loan(Loan),
        FD(FD),
        Forex(Forex)
    }

    #[derive(Debug,Clone, Copy,Deserialize,Serialize)]
    pub enum Customer_t{
        Normal,
        HNI,
        FrequentUser,
        NonFrequentUser
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct Account{
        account_id: Uuid,
        customer: Customer,
        acc_type: Account_T,
        product_conf: ProductConfig
    }


    #[derive(Debug,Clone, Copy,Serialize,Deserialize)]
    pub enum Account_T{
        Savings,
        Current,
        Checking,
        Deposit,
    }

    #[derive(Debug,Clone, Copy,Serialize,Deserialize)]
    pub enum ProductConfig{
        Account{min_balance: Money,pricing: Money},
        Loan{amount: Money, tenure: i64, interest:f32},
        FD{amount: Money, tenure: i64, interest:f32},
        Forex{amt: Money,to_convert: Currency}
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AccountBalance {
        /// Posted balance according to the account ledger.
        pub booked: Money,
        /// Amount currently available for spending or withdrawal.
        pub available: Money,
        /// Funds reserved for pending transactions or authorizations.
        pub held: Money,
        /// Pending credits/debits, tracked separately from posted entries.
        pub pending: Money,
        pub as_of: DateTime<Utc>,
    }

    impl AccountBalance{
        pub fn new(curr: Option<Currency>) -> AccountBalance{
            let curr = curr.unwrap_or_default();
            Self { booked: Money::new(0, curr), available: Money::new(0, curr), held: Money::new(0,curr), pending: Money::new(0, curr), as_of: Utc::now() }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Currency {
        INR, // 2 dp
        USD, // 2 dp
        JPY, // 0 dp
        BTC, // 9 dp
        KWD  // 3 dp
    }

    impl Default for Currency{
        fn default() -> Self {
            Self::INR
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Money {
        pub units: i64,
        pub currency: Currency,
    }

    impl Money{
        pub fn new(units: i64,currency: Currency) -> Self{  
            Self { units, currency }
        }
    }

    impl Default for Money{
        fn default() -> Self {
            Self { units: 0, currency: Currency::INR }
        }
    }


    #[derive(Debug,Serialize,Deserialize)]
    pub struct Loan{
        pub loan_id: Uuid,
        pub customer_id: Uuid,
        pub amount: Money,
        pub tenure_months: i64,
        pub interest_rate: f32,
        pub emi: Money,
        pub outstanding_principal: Money,
        pub status: LoanStatus,
        pub start_date: DateTime<Utc>,
        pub end_date: DateTime<Utc>,
        pub next_emi_date: DateTime<Utc>,
    }

    impl Loan{
        pub fn new(customer_id: Uuid, config: &ProductConfig) -> Self{
            if let ProductConfig::Loan{amount, tenure, interest} = config{
                let start_date = Utc::now();
                let end_date = start_date + chrono::Duration::days(*tenure * 30);
                let monthly_interest = interest / 100.0 / 12.0;  // Compounding period per month
                let emi_amount = (amount.units as f64 * monthly_interest as f64 * (1.0 + monthly_interest as f64).powi(*tenure as i32)) / ((1.0 + monthly_interest as f64).powi(*tenure as i32) - 1.0);
                Self {loan_id: Uuid::now_v7(),customer_id,amount: *amount,tenure_months: *tenure,interest_rate: *interest,emi: Money::new(emi_amount as i64, amount.currency),outstanding_principal: *amount,status: LoanStatus::Active,start_date,end_date,next_emi_date: start_date + chrono::Duration::days(30)}
            } else {
                panic!("Invalid ProductConfig for Loan")
            }
        }
        
        /// Expected converted payment
        pub fn pay_emi(&mut self, payment: Money) -> Result<(), String>{
            if payment.units >= self.emi.units{
                self.outstanding_principal.units -= payment.units;
                self.next_emi_date = self.next_emi_date + chrono::Duration::days(30);
                if self.outstanding_principal.units <= 0{ self.status = LoanStatus::Paid; }
                Ok(())
            } else {
                Err("Payment amount is less than EMI".to_string())
            }
        }
    }

    #[derive(Debug,Clone, Copy,Serialize,Deserialize)]
    pub enum LoanStatus{
        Active,
        Paid,
        Defaulted,
        Closed,
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct FD{
        pub fd_id: Uuid,
        pub customer_id: Uuid,
        pub principal: Money,
        pub tenure_months: i64,
        pub interest_rate: f32,
        pub maturity_amount: Money,
        pub status: FDStatus,
        pub start_date: DateTime<Utc>,
        pub maturity_date: DateTime<Utc>,
        pub is_auto_renew: bool,
    }

    impl FD{
        pub fn new(customer_id: Uuid, config: &ProductConfig, auto_renew: bool) -> Self{
            if let ProductConfig::FD{amount, tenure, interest} = config{
                let start_date = Utc::now();
                let maturity_date = start_date + chrono::Duration::days(*tenure * 30);
                let maturity_amount_units = amount.units as f64 * (1.0 + (*interest as f64  / 100.0) * (*tenure as f64 / 12.0));
                Self {fd_id: Uuid::now_v7(),customer_id,principal: *amount,tenure_months: *tenure,interest_rate: *interest,maturity_amount: Money::new(maturity_amount_units as i64, amount.currency),status: FDStatus::Active,start_date,maturity_date,is_auto_renew: auto_renew}
            } else {
                panic!("Invalid ProductConfig for FD")
            }
        }

        pub fn mature(&mut self){ self.status = FDStatus::Matured; }

        pub fn withdraw(&mut self) -> Result<Money, String>{
            match self.status{
                FDStatus::Matured => {
                    self.status = FDStatus::Withdrawn;
                    Ok(self.maturity_amount)
                },
                FDStatus::Active => Err("FD has not matured yet".to_string()),
                FDStatus::Withdrawn => Err("FD already withdrawn".to_string()),
                FDStatus::Closed => Err("FD is closed".to_string()),
            }
        }
    }

    #[derive(Debug,Clone, Copy,Serialize,Deserialize)]
    pub enum FDStatus{
        Active,
        Matured,
        Withdrawn,
        Closed,
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct Forex{
        pub forex_id: Uuid,
        pub customer_id: Uuid,
        pub from_amount: Money,
        pub to_currency: Currency,
        pub to_amount: Money,
        pub exchange_rate: f64,
        pub status: ForexStatus,
        pub transaction_date: DateTime<Utc>,
    }

    impl Forex{
        pub fn new(customer_id: Uuid, config: &ProductConfig, exchange_rate: f64) -> Self{
            if let ProductConfig::Forex{amt, to_convert} = config{
                let to_amount_units = (amt.units as f64 * exchange_rate) as i64;
                Self {forex_id: Uuid::now_v7(),customer_id,from_amount: *amt,to_currency: *to_convert,to_amount: Money::new(to_amount_units, *to_convert),exchange_rate,status: ForexStatus::Pending,transaction_date: Utc::now()}
            } else {
                panic!("Invalid ProductConfig for Forex")
            }
        }
        pub fn complete(&mut self){ self.status = ForexStatus::Completed; }
        pub fn cancel(&mut self){ self.status = ForexStatus::Cancelled; }
    }

    #[derive(Debug,Clone, Copy,Serialize,Deserialize)]
    pub enum ForexStatus{
        Pending,
        Completed,
        Cancelled,
        Failed,
    }

}