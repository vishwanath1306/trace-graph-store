use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personal{
    pub name: String,
    pub goal: String,
    pub age: u8,
    pub visited: Vec<String>,
    pub extra: Option<String>
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct JaegerRoot{
//     pub data: Vec<>,
//     pub total: u64,
//     pub limit: u64,
//     pub offset: u64, 
//     pub errors: Option<>
// }