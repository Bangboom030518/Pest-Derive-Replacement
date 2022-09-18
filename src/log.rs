use std::fs;

const LOG_FILE: &str = "log.log";

const LOG_ERROR: &str = "Couldn't write to log file";

pub fn log(content: &str) {
    let file_content = fs::read_to_string(LOG_FILE).unwrap();
    fs::write(LOG_FILE, format!("{}\n{}", content, file_content)).expect(LOG_ERROR);
}

pub fn clear() {
    fs::write(LOG_FILE, "").expect(LOG_ERROR);
}
