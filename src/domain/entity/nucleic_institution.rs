use serde::{Deserialize, Serialize};
use mybatis::DateTimeNative;

pub struct NucleicInstitution {
    // id
    pub id: Option<String>,
    // name
    pub institution_name: Option<String>,
    // address
    pub institution_address: Option<String>,
    // phone
    pub institution_phone: Option<String>,
}