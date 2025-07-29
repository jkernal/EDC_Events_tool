//FILENAME: main.rs
//AUTHOR: Jonathan Shambaugh
//PURPOSE: To find all the address comment pairs from the toyopuc comment file csv and screenworks export csv files.
//Then output a binary file so the accompanying python script can load it to a dictionary and execute fast lookups.
//NOTES: 
//VERSION: 0.1.0
//DEPENDENCIES:
//csv = "1.3"
//encoding_rs = "0.8"
//encoding_rs_io = "0.1"


//imports
use std::env;
use std::time::Instant;
use std::{fs::{self, File}, io::{BufReader, Write}, path::{PathBuf}};
use csv::{ReaderBuilder};
use encoding_rs::UTF_16LE;
use encoding_rs_io::DecodeReaderBytesBuilder;
#[allow(unused_imports)]
use log::{info, warn, error, debug, trace};
use crate::{config::load_config, global_logger::init_logger};
mod global_logger;
mod config;



fn get_first_file(dir_path: &str) -> Option<PathBuf> {
    /*
    Get the path to the first file in the directory.

    Arguments:
        dir_path (&str): a string that is a path to a directory.

    Returns:
        Option<PathBuf>: the path to the first file in the given directory.
     */
    
    //Read the directory
    let entries_result = fs::read_dir(dir_path);

    //Handle reading errors - if there are any errors I assume its an empty directory.
    let entries = match entries_result {
        Ok(dir_rdr) => dir_rdr,
        Err(error) => {
            error!("{error}");
            return None;
        }
    };

    //loop through the array of entries and return the first file. (there could be directories in the entries array)
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            return Some(path);
        }
        
    }

    //if the code gets here no files were found.
    None
}



fn pad_or_truncate(input: &str, size: usize) -> Vec<u8> {
    /*
    Check to ensure the u8 array matches the given size, if it is too large it will truncate,
    if it is too small it will add to it to match the size.

    Arguments:
        input (&str): the string to be check and modified if needed.
        size (usize): how long the string needs to be
    
    Returns:
        Vec<u8>: an array of the bytes sized to match the size argument.
     */
    
    //convert the input string into an array of bytes
    let mut bytes = input.as_bytes().to_vec();

    //check if bytes meet size requirements
    if bytes.len() > size {
        bytes.truncate(size);
    } else if bytes.len() < size {
        bytes.resize(size, b' ');
    }
    
    bytes

}



fn get_screenwroks_comments(csv_path: &str, addr_index: usize, comment_index: usize) -> Vec<Vec<u8>> {
    /*
    Get all if the address and comments in the screenworks export csv and put them in an array.

    Arguments:
        csv_path (&str): the path to the screenworks csv.
        addr_index (usize): the position of the address in the csv starting at 0.
        comment_index (usize): the position of the comment in the csv starting at 0.

    Returns:
        Vec<Vec<u8>>: the complete 2 dimensional array of addresses and comments. The address field is 64 bytes long and the comment field is 96 bytes long.
            example:
                [[GMF900, L/C FAULT (OPERATOR SIDE)], [EM1A0, ENDURANCE]]
     */

    //Open the file
    let file_result = File::open(csv_path);

    //Handle opening errors
    let csv_file = match file_result  {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the csv file.\n{error}")
    };

    //The ScreenWorks software exports csv in UTF-16 LE so a conversion is needed:
    //Covert UTF-16 LE to UTF-8
    let converted_csv = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_16LE))
        .build(csv_file);

    //Construct a reader object
    let reader = BufReader::new(converted_csv);

    //Construct the CSV reader object
    let mut csv_reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(reader);

    //Create iterator for csv file
    let iter = csv_reader.records();

    //loop and write all address and comment pairs to the array
    let mut records = Vec::new();
    let mut address;
    let mut comment;
    let mut addr_field;
    let mut comment_field;
    let mut record_count = 0;
    for line in iter {
        //Handle string errors
        let record = match line {
            Ok(rec) => rec,
            Err(error) => {
                debug!("Error reading this line: {error}");
                continue; //Skip errored line
            }
        };
        
        //check to make sure the indices are accessible
        if record.len() <= comment_index || record.len() <= addr_index {
            debug!("Skipping malformed or short record: {record:?}");
            continue;
        }

        //Since _record is a Options enum, have to check for error condition
        //If the line errors out, I assume that there is no usable address, so I skip it
        if let Some(field) = record.get(addr_index) {
            //remove spaces from start and end of the string
            address = field.trim();
        } else {
            continue;
        }

        //splits the string by \
        address = address.split("\\").next().unwrap_or("XXXXX");// the split method returns an iterator so the next() method selects the first element

        //fallback upon error to empty string
        let raw_comment = record.get(comment_index).unwrap_or("Not found");
        //replace \n characters with a space
        let replaced_comment = raw_comment.replace("\\n", " ");
        //remove spaces from start and end
        comment = replaced_comment.trim().to_string();

        debug!("{address} = {comment}");

        //put the address string in a 96 byte field, left justified
        addr_field = format!("{address:<64}");
        //put the comment string in a 96 byte field, left justified
        comment_field = format!("{comment:<96}");

        //confirm or correct field size then combine the two field to complete an address and comment pair
        let mut record_bytes = pad_or_truncate(&addr_field, 64);
        record_bytes.extend(pad_or_truncate(&comment_field, 96));

        debug!("Record len: {}", record_bytes.len());

        //add the complete record to the records array
        records.push(record_bytes);
        record_count += 1;

    }

    //log how many records were found
    if record_count > 1 {
        info!("Found {record_count} comments in the ScreenWorks csv file.");
    } else if record_count == 1 {
        info!("Found {record_count} comment in the ScreenWorks csv file.");
    } else {
        info!("No comments found in the ScreenWorks csv file.");
    }

    records

}



fn get_toyopuc_comments(csv_path: &str, addr_index: usize, comment_index: usize) -> Vec<Vec<u8>> {
    /*
    Get all if the address and comments in the toyopuc comment csv and put them in an array.

    Arguments:
        csv_path (&str): the path to the toyopuc csv.
        addr_index (usize): the position of the address in the csv starting at 0.
        comment_index (usize): the position of the comment in the csv starting at 0.

    Returns:
        Vec<Vec<u8>>: the complete 2 dimensional array of addresses and comments. The address field is 64 bytes long and the comment field is 96 bytes long.
            example:
                [[GMF900, L/C FAULT (OPERATOR SIDE)], [GMF8F0, PART SET FAULT]]
     */
    
    //Open the file
    let file_result = File::open(csv_path);

    //Handle opening errors
    let csv_file = match file_result  {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the csv file.\n{error}")
    };

    //Construct a reader object
    let reader = BufReader::new(csv_file);

    //Construct the CSV reader object
    let mut csv_reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(reader);

    //Create iterator for csv file
    let iter = csv_reader.records();

    //loop and write all address and comment pairs to the array
    let mut records = Vec::new();
    let mut address;
    let mut comment;
    let mut addr_field;
    let mut comment_field;
    let mut record_count = 0;
    for line in iter {
        //Handle string errors
        let record = match line {
            Ok(rec) => rec,
            Err(error) => {
                debug!("Error reading this line: {error}");
                continue; //Skip errored line
            }
        };
        
        //Check to make sure the indices are accessible
        if record.len() <= comment_index || record.len() <= addr_index {
            debug!("Skipping malformed or short record: {record:?}");
            continue;
        }

        //Since _record is a Options enum, have to check for error condition
        //If the line errors out, I assume that there is no usable address, so I skip it
        if let Some(field) = record.get(addr_index) {
            //remove spaces from start and end of the string
            address = field.trim();
        } else {
            continue;
        }

        //splits the string by \
        address = address.split("\\").next().unwrap_or("XXXXX");// the split method returns an iterator so the next() method selects the first element

        //fallback upon error to empty string
        let raw_comment = record.get(comment_index).unwrap_or("Not found");
        //replace \n characters with a space
        let replaced_comment = raw_comment.replace("\\n", " ");
        //remove spaces from start and end
        comment = replaced_comment.trim().to_string();

        debug!("{address} = {comment}");

        //put the address string in a 96 byte field, left justified
        addr_field = format!("{address:<64}");
        //put the comment string in a 96 byte field, left justified
        comment_field = format!("{comment:<96}");

        //confirm or correct field size then combine the two field to complete an address and comment pair
        let mut record_bytes = pad_or_truncate(&addr_field, 64);
        record_bytes.extend(pad_or_truncate(&comment_field, 96));

        debug!("Record len: {}", record_bytes.len());

        //add the complete record to the records array
        records.push(record_bytes);
        record_count += 1;

    }

    //log how many records were found
    if record_count > 1 {
        info!("Found {record_count} comments in the Toyopuc csv file.");
    } else if record_count == 1 {
        info!("Found {record_count} comment in the Toyopuc csv file.");
    } else {
        info!("No comments found in the Toyopuc csv file.");
    }

    records

}



fn write_comments_table_file(contents: Vec<Vec<u8>>, bin_path: &str) -> bool {
    /*
    Takes the given contents and writes to the output path given as a binary file.

    Arguments:
        contents (Vec<Vec<u8>>): the two dimensional array that will be written to the file.
        bin_path (&str): the path to the file to be written to, if one already exists, it will be overwritten.
    
    Returns:
        bool: Weather or not the write was successful.
    */

    //create .bin file
    let output_file_result = File::create(bin_path);

    //handle file creation errors
    let mut output_file = match output_file_result {
        Ok(file) => file,
        Err(error) => {
            error!("An error occurred when trying to create/open {bin_path}.\n{error}");
            return false;
        }
    };

    //write all elements in the array
    for chunk in contents {

        match output_file.write_all(&chunk) {
            Ok(_) => {},
            Err(error) => {
                error!("Failed to write data: {error}");
                continue;
            }
        }
    }

    true //indicate that the write was successful

}



fn main() {
    /*
    Loads configuration, initializes logging, and processes ScreenWorks and Toyopuc CSV comment files.

    This function performs the following:
    - Loads user configuration from a config file.
    - Initializes a rotating file logger.
    - Deletes existing output `.bin` files if they exist.
    - Searches for the first CSV file in the configured ScreenWorks and Toyopuc input directories.
    - Extracts address-comment pairs from these CSV files using configured column indexes.
    - Writes the parsed data to binary output files for use by downstream tools or processes.
    - Logs the success or failure of each step.

    Arguments:
    None

    Returns:
    None
    */

    // Start the timer to measure execution time
    let start = Instant::now();

    //get the current exe directory
    let exe_path = match env::current_exe(){
        Ok(path) => path,
        Err(error) => {
            println!("Failed to get current executable path: {error}");
            return;
        }
    };
    
    let exe_dir = match exe_path.parent() {
        Some(path) => path,
        None => {
            println!("Failed to get parent directory of the executable path.");
            return;
        }
    };

    //create the path to the config file
    let config_path = exe_dir.join("config.toml");

    //convert the config_path to a string
    let config_path_str = match config_path.to_str() {
        Some(path_str) => path_str,
        None => {
            println!("Failed to convert executable directory to string.");
            return;
        }
    };

    //load the user config
    let user_config = load_config(config_path_str);

    //initialize the global logger
    init_logger(&user_config.log_level, "address_comment_loader");

    debug!("Working directory: {}", exe_dir.display());
    debug!("User config: {user_config:?}");

    //convert the user_config sw_output_path to a PathBuf type
    let sw_out_path = PathBuf::from(&user_config.sw_output_path);

    //check to see of the screenworks output file exists
    if sw_out_path.is_file() {
        match fs::remove_file(&sw_out_path) {
            Ok(_) => {},
            Err(error) => {
                let path_str = sw_out_path.to_string_lossy();
                info!("Failed to delete {path_str} | {error}");
            }
        }
    }

    //convert the user_config toyo_output_path to a PathBuf type
    let toyo_out_path = PathBuf::from(&user_config.toyo_output_path);

    //check to see of the toyopuc output file exists
    if toyo_out_path.is_file() {
        match fs::remove_file(&toyo_out_path) {
            Ok(_) => {},
            Err(error) => {
                let path_str = toyo_out_path.to_string_lossy();
                info!("Failed to delete {path_str} | {error}");
            }
        }
    }

    let screenworks_csv_exists: bool;

    //get the pathbuf of the screenworks csv file
    let screenworks_csv_pathbuf = match get_first_file(&user_config.screenworks_csv_dir) {
        Some(path) => {
            screenworks_csv_exists = true;
            path
        }
        None => {
            info!("ScreenWorks csv doesn't exist.");
            screenworks_csv_exists = false;
            PathBuf::new()
        }
    };

    //convert the pathbuf to a string
    let screenworks_csv_path = screenworks_csv_pathbuf.to_str().unwrap_or("Not found");

    let toyopuc_csv_exists: bool;

    //get the pathbuf to the toyopuc csv
    let toyopuc_csv_pathbuf = match get_first_file(&user_config.toyopuc_csv_dir) {
        Some(path) => {
            toyopuc_csv_exists = true;
            path
        },
        None => {
            info!("Toyopuc csv doesn't exist.");
            toyopuc_csv_exists = false;
            PathBuf::new()
        }
    };

    //convert the pathbuf to a string
    let toyopuc_csv_path = toyopuc_csv_pathbuf.to_str().unwrap_or("Not found");

    let mut sw_file_write_complete = false;

    //if screenworks csv does exists, get the comments and write them to the screenworks output file
    if screenworks_csv_exists {
        let sw_comments_table = get_screenwroks_comments(
        screenworks_csv_path,
        user_config.sw_addr_col - 1,
        user_config.sw_comment_col - 1);

        sw_file_write_complete = write_comments_table_file(sw_comments_table,
            &user_config.sw_output_path);
    }

    let mut toyo_file_write_complete = false;

    //if toyopuc csv exists, get the comments and write them to the toyopuc output file
    if toyopuc_csv_exists {
        let toyo_comments_table = get_toyopuc_comments(
        toyopuc_csv_path,
        user_config.toyo_addr_col - 1,
        user_config.toyo_comment_col - 1);
        
        toyo_file_write_complete = write_comments_table_file(toyo_comments_table,
            &user_config.toyo_output_path)
    }

    //log what was completed
    if sw_file_write_complete && toyo_file_write_complete {
        info!("Wrote {} file and {} file.", &user_config.sw_output_path, &user_config.toyo_output_path);
    } else if sw_file_write_complete {
        info!("Wrote {} file.", &user_config.sw_output_path);
    } else if toyo_file_write_complete {
        info!("Wrote {} file.", &user_config.toyo_output_path);
    } else {
        info!("No files wrote. Something might be wrong.");
    }

    //log the execution time
    let elapsed = start.elapsed();
    info!("Execution time: {elapsed:.2?}");
}