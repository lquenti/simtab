// TODO quote handling

use anyhow::{bail, Result};

#[derive(Debug)]
struct Table {
    xss: Vec<Row>,
}
type Row = Vec<Cell>;
type Cell = String;

fn line_is_md_sep(line: &str) -> bool {
    false
}

impl Table {
    pub fn from_csv(csv_str: &str) -> Self {
        Self {
            xss: csv_str
                .split("\n")
                .map(|line| line.split(",").map(|x| x.to_owned()).collect::<Vec<Cell>>())
                .collect(),
        }
    }
    pub fn to_csv(self) -> String {
        self.xss
            .iter()
            .map(|line| line.join(","))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn from_md_table(md_table_str: &str) -> Result<String> {
        let lines: Vec<&str> = md_table_str.split("\n").collect();

        // check if at least 2 lines
        // (header and seperator)
        if lines.len() < 2 {
            let error_msg = "Markdown table too small to be valid!";
            log::error!("{}", error_msg);
            bail!(Err(error_msg));
        }

        // Check if second line is a seperator
        // (unwrap is fine, we checked b4)
        let hopefully_sep = lines.get(1).unwrap();
        if !line_is_md_sep(hopefully_sep) {
            let error_msg = format!(
                "Markdown seperator is missing in line 2!\nInstead: \"{}\"",
                hopefully_sep
            );
            log::error!("{}", error_msg);
            bail!(Err(error_msg));
        }

        // parse header
        // remove first and last "|" since they are a the edge
        let header: Row = lines.get(0).unwrap()[1..]
            .split("\n")
            .map(|x| x.to_owned())
            .collect();

        // parse body

        Ok(String::new())
    }

    pub fn to_md_table(self) -> String {
        String::new()
    }
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    println!("Hello, world!");
}
