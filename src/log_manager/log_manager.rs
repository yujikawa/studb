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