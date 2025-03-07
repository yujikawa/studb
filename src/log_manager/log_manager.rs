use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct LogManager {
    log_file: File,
}

impl LogManager {
    pub fn new(log_filename: &str) -> io::Result<Self> {
        let log_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(log_filename)?;

        Ok(LogManager { log_file })
    }

    pub fn write_log(&mut self, log: &str) -> io::Result<()> {
        writeln!(self.log_file, "{}", log)?;
        self.log_file.flush()?;
        Ok(())
    }

    pub fn read_logs(&mut self) -> io::Result<Vec<String>> {
        let mut logs = Vec::new();
        self.log_file.seek(SeekFrom::Start(0))?;

        let mut buffer = String::new();
        self.log_file.read_to_string(&mut buffer)?;

        for line in buffer.lines() {
            logs.push(line.to_string());
        }

        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_manager() {
        let log_filename = "test_log.studb";
        let mut log_manager = LogManager::new(log_filename).unwrap();

        log_manager.write_log("BEGIN TRANSACTION").unwrap();
        log_manager
            .write_log("UPDATE users SET name='Alice' WHERE id = 1")
            .unwrap();
        log_manager.write_log("COMMIT").unwrap();

        let logs = log_manager.read_logs().unwrap();
        assert_eq!(
            logs,
            vec![
                "BEGIN TRANSACTION",
                "UPDATE users SET name='Alice' WHERE id = 1",
                "COMMIT"
            ]
        );
        fs::remove_file(log_filename).unwrap();
    }
}
