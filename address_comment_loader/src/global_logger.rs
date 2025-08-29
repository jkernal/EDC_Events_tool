//FILENAME: global_logger.rs
//AUTHOR: Jonathan Shambaugh
//PURPOSE: initialize a global logger.
//NOTES: 
//DEPENDENCIES:
//log = "0.4"
//flexi_logger = "0.27"


use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use std::sync::Once;


static INIT: Once = Once::new();


pub fn init_logger(level: &str, basename: &str) {
    
    let level = level.to_string();
    let basename = basename.to_string();
    //ensures it can only be initialized once
    INIT.call_once(move || {
        Logger::try_with_str(level)
            .unwrap()
            //settings for the format of the output
            .format(|writer, now, record| {
                write!(
                    writer,
                    "[{}] [{}] - {}",
                    record.level(),
                    now.now().format("%Y-%m-%d %H:%M:%S"),
                    &record.args()
                )
            })
            //settings for file
            .log_to_file(
                FileSpec::default()
                    .basename(basename)
                    .suffix("log")
                    .directory("log_files"),
            )
            //settings for max file size and rotating between log files
            .rotate(
                Criterion::Size(26_214_400), // 25 MB max file size
                Naming::Timestamps,         // Use timestamped rotated files
                Cleanup::KeepLogFiles(20),    // Keep last 20 logs
            )
            .start()
            .unwrap();
    });
}
