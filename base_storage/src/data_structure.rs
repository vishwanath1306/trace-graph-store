use serde::{de, Serialize, Deserialize, Deserializer};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagValue {
    Float(f64),
    String(String),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct References{
    #[serde(rename(deserialize = "refType"))]
    pub ref_type: String,

    #[serde(rename(deserialize = "traceID"))]
    pub trace_id: String,

    #[serde(rename(deserialize = "spanID"))]
    pub span_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags{
    pub key: String,

    #[serde(rename(deserialize = "type"))]
    pub type_: String,

    #[serde(deserialize_with = "value_to_string")]
    pub value: String,
}

fn value_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>{
        let s: TagValue = de::Deserialize::deserialize(deserializer)?;
        let result = match s {
            TagValue::Float(value) => {
                Ok(value.to_string())
            }, 

            TagValue::String(value) => {
                Ok(value)
            }
        };
        result

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span{

    #[serde(rename(deserialize = "traceID"))]
    pub trace_id: String,

    #[serde(rename(deserialize = "spanID"))]
    pub span_id: String, 

    pub flags: u8,

    #[serde(rename(deserialize = "operationName"))]
    pub operation_name: String, 

    pub references: Vec<Option<References>>,
    
    #[serde(rename(deserialize = "startTime"))]
    pub start_time: u128,

    pub duration: u128,

    pub tags: Vec<Tags>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data{
    #[serde(rename(deserialize = "traceID"))]
    pub trace_id: String, 

    pub spans: Vec<Span>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaegerRoot{
    pub data: Vec<Data>,

    pub total: u64,

    pub limit: u64,

    pub offset: u64, 

    pub errors: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personal{
    pub name: String,
    pub goal: String,
    pub age: u8,
    pub visited: Vec<String>,
    pub extra: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personal2{
    pub name: String,
    pub goal: String,
    pub age: u8,
    pub visited: Vec<String>,
    pub extra: Option<String>,
    pub address: Address
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address{
    pub street: String, 
    pub state: String, 
    pub city: String
}
