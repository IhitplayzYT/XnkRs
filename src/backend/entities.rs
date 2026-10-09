pub mod Entities{
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid, timestamp::UUID_TICKS_BETWEEN_EPOCHS};
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
        pub fn new(email:String,phone: String,is_customer: bool,ldbid: Option<Uuid>,ws_id: Option<Uuid>) -> Self{
            let user = User::new(email, phone, is_customer);
            let (uuid,ldbid,ws_id) = (user.get_id(),ldbid.unwrap_or(Uuid::new_v4()),ws_id.unwrap_or(Uuid::new_v4()));
            Self { user,user_metadata: UserMetaData::default() , audit_fields:Audit::new(uuid, ldbid, ws_id),customer_type: Customer_t::Normal,relationships:vec![]}   
        }

        /// Create new user from the individual components if present
        pub fn new_from(user: User,user_metadata: UserMetaData,audit_fields: Audit,customer_type: Customer_t) -> Self{
            Self { user, user_metadata, audit_fields,customer_type,relationships:vec![]}
        }
    }



    /// Validate international phone number standards
    /// v2 api used Internally to validate phone numbers
    fn validate_phone_no_v2(phone: &str) -> Result<(), ValidationError> {
        match phonenumber::parse(None, phone) {
            Ok(number) if phonenumber::is_valid(&number) => Ok(()),
            _ => Err(ValidationError::new("invalid_phone_number")),
        }
    }



    #[deprecated]
    /// Internal validation function to validate phone numbers
    /// v1 supporting simple logic
    fn validate_phone_no_v1(phone: &str) -> Result<(),ValidationError>{
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
        Account,
        Loan,
        FD,
        Forex
    }

    #[derive(Debug,Clone, Copy,Deserialize,Serialize)]
    pub enum Customer_t{
        Normal,
        HNI,
        FrequentUser,
        NonFrequentUser
    }



}