use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) {
    for line in reader.lines().filter_map(|result| result.ok()) {
        match_line(&line, &pattern, &mut writer);
    }
}

pub fn match_line(line: &str, pattern: &str, mut writer: impl std::io::Write) {
    if line.contains(pattern) {
        writeln!(writer, "{}", line);
    }
}
