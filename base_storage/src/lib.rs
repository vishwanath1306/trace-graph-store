pub mod data_structure;

use std::fs;
use data_structure::{JaegerRoot, Personal, Personal2};

use postgres::Client;


pub fn jaeger_parser(filename: String) -> JaegerRoot{
    let file_descriptor = fs::File::open(filename).expect("File should be opened in read only");
    let jaeger_base: JaegerRoot = serde_json::from_reader(file_descriptor).expect("File should be proper JSON");
    jaeger_base
}

pub fn postgres_bridge(){
    // Figure out the different parameters you need
    // Ideally should be for setting up the 
    let client = Client::connect("postgresql://vishwanath:beyblade@localhost/sample_db", postgres::NoTls);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_jaeger_base(){
        let jaeger_base_file = "./example_json/example_jaeger_trace.json";
        let jaeger_details = jaeger_parser(jaeger_base_file.to_string());
        println!("The jaeger data for 1 is: {:?}", jaeger_details.data[0].spans[0]);
        assert_eq!(0, jaeger_details.total);
    }

    #[test]
    pub  fn test_sql_table(){
        postgres_bridge();
    }
}
