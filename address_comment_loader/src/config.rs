//FILENAME: config.rs
//AUTHOR: Jonathan Shambaugh
//PURPOSE: get the config from a config.toml file.
//NOTES: 
//DEPENDENCIES:
//serde = { version = "1.0", features = ["derive"] }
//config = "0.14"


use serde::Deserialize;


//Allows you to use println!("{:?}", my_struct) or dbg!(my_struct) to print it for debugging.
#[derive(Debug, Deserialize)]

//configure what variables are read
pub struct Settings {
    pub log_level: String,
    pub screenworks_csv_dir: String,
    pub toyopuc_csv_dir: String,
    pub sw_output_path: String,
    pub sw_addr_col: usize,
    pub sw_comment_col: usize,
    pub toyo_output_path: String,
    pub toyo_addr_col: usize,
    pub toyo_comment_col: usize,
}

//opens file and reads variables into a config object
pub fn load_config(path: &str) -> Settings {
    let builder = config::Config::builder()
        .add_source(config::File::with_name(path)) // Load the config file
        .build()
        .expect("Failed to build config");

    builder
        .try_deserialize::<Settings>()
        .expect("Failed to deserialize config")
}
