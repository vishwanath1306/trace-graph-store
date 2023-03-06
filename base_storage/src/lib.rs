pub mod data_structure;

use std::fs;
use data_structure::JaegerRoot;


pub fn jaeger_parser(filename: String) -> JaegerRoot{
    let file_descriptor = fs::File::open(filename).expect("File should be opened in read only");
    let jaeger_base: JaegerRoot = serde_json::from_reader(file_descriptor).expect("File should be proper JSON");
    jaeger_base
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_jaeger_base(){
        let jaeger_base_file = "./example_json/example_jaeger_trace.json";
        let jaeger_details = jaeger_parser(jaeger_base_file.to_string());
        // println!("The jaeger details are: {:?}", jaeger_details);
        println!("The jaeger data for 1 is: {:?}", jaeger_details.data[0].spans[0]);
        assert_eq!(0, jaeger_details.total);
    }
}
