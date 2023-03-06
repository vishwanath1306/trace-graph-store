pub mod data_structure;

use std::fs;
use data_structure::Personal;

pub fn parse_personal_json_file(filename: String) -> Personal{

    let file_descriptor = fs::File::open(filename).expect("File should be opened in read only");
    let person_details: Personal = serde_json::from_reader(file_descriptor).expect("ile should be proper JSON");
    person_details
}


pub fn jaeger_parser(filename: String){

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
    
}
