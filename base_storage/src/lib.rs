pub mod data_structure;

use std::fs;
use data_structure::{JaegerRoot, Personal, Personal2};


pub fn jaeger_parser(filename: String) -> JaegerRoot{
    let file_descriptor = fs::File::open(filename).expect("File should be opened in read only");
    let jaeger_base: JaegerRoot = serde_json::from_reader(file_descriptor).expect("File should be proper JSON");
    jaeger_base
}

pub fn parse_personal_json_file(filename: String) -> Personal{

    let file_descriptor = fs::File::open(filename).expect("File should be opened in read only");
    let person_details: Personal = serde_json::from_reader(file_descriptor).expect("ile should be proper JSON");
    person_details
}

pub fn parse_personal_with_address_json_file(filename: String) -> Personal2{

    let file_descriptor = fs::File::open(filename).expect("File should be opened in read only");
    let person_details: Personal2 = serde_json::from_reader(file_descriptor).expect("ile should be proper JSON");
    person_details
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_personal_json_parser(){
        let example_file = "./example_json/example_personal.json";
        let personal_details = parse_personal_json_file(example_file.to_string());
        let visited_string = vec!["SFO".to_string(), "NYC".to_string(), "ATL".to_string(), "BOL".to_string()];
        assert_eq!("John", personal_details.name);
        assert_eq!(26, personal_details.age);
        assert_eq!(visited_string, personal_details.visited);
        assert_eq!(None, personal_details.extra);
    }

    #[test]
    pub fn test_personal_json_with_string(){
        let example_file = "./example_json/example_personal_string.json";
        let visited_string = vec!["SFO".to_string(), "NYC".to_string(), "ATL".to_string(), "BOL".to_string()];
        let personal_details = parse_personal_json_file(example_file.to_string());
        assert_eq!("John", personal_details.name);
        assert_eq!(26, personal_details.age);
        assert_eq!(visited_string, personal_details.visited);
        assert_eq!("Hello World", personal_details.extra.unwrap());
    }

    #[test]
    pub fn test_personal_address_with_string(){
        let example_file = "./example_json/nested_json.json";
        let visited_string = vec!["SFO".to_string(), "NYC".to_string(), "ATL".to_string(), "BOL".to_string()];
        let personal_details: Personal2 = parse_personal_with_address_json_file(example_file.to_string());
        assert_eq!("John", personal_details.name);
        assert_eq!(26, personal_details.age);
        assert_eq!(visited_string, personal_details.visited);
        assert_eq!("Hello World", personal_details.extra.unwrap());
        assert_eq!("Manhattan", personal_details.address.city);
    }

    #[test]
    pub fn test_jaeger_base(){
        let jaeger_base_file = "./example_json/example_jaeger_trace.json";
        let jaeger_details = jaeger_parser(jaeger_base_file.to_string());
        println!("The jaeger data for 1 is: {:?}", jaeger_details.data[0].spans[0]);
        assert_eq!(0, jaeger_details.total);
    }
}
