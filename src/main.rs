// TODO put csv and md line parser into its own func

use anyhow::{bail, Result};

#[derive(Debug)]
struct Table {
    #[allow(dead_code)]
    xss: Vec<Row>,
}
type Row = Vec<Cell>;
type Cell = String;

// TODO make module for this
fn md_enough_lines(lines: &[&str]) -> Result<()> {
    // check if at least 2 lines (header and seperator)
    // TODO make macro for this
    if lines.len() < 2 {
        let error_msg = "Markdown table too small to be valid!";
        log::error!("{}", error_msg);
        bail!(error_msg);
    }
    Ok(())
}

fn md_all_lines_contain_same_amount_of_columns(lines: &[&str]) -> Result<()> {
    let counts = lines
        .iter()
        .map(|x| x.matches('|').count())
        .collect::<Vec<usize>>();
    let res = counts[1..].iter().all(|x| *x == counts[0]);
    // TODO make macro for this
    if !res {
        let error_msg = "Markdown tables don't contain same amount of cells!";
        log::error!("{}", error_msg);
        bail!(error_msg);
    }
    Ok(())
}

fn md_has_valid_header(line: &str) -> Result<()> {
    let valid_chars: [char; 2] = ['|', '-'];
    let res = line.chars().all(|x| valid_chars.contains(&x));
    // TODO make macro for this
    if !res {
        let error_msg = "Header is not valid!";
        log::error!("{}", error_msg);
        bail!(error_msg);
    }
    Ok(())
}

fn md_lines_are_valid(lines: &[&str]) -> Result<()> {
    md_enough_lines(lines)?;
    md_all_lines_contain_same_amount_of_columns(lines)?;
    md_has_valid_header(lines[1])?;
    Ok(())
}

fn md_line_to_row(line: &str) -> Row {
    let cells = line.split('|').collect::<Vec<&str>>();
    // first and last are seps
    cells[1..cells.len() - 1]
        .iter()
        .map(|x| (*x).to_owned())
        .collect::<Vec<String>>()
}

impl Table {
    #[allow(dead_code)]
    pub fn from_csv(csv_str: &str) -> Self {
        Self {
            xss: csv_str
                .split('\n')
                .map(|line| line.split(',').map(|x| x.to_owned()).collect::<Vec<Cell>>())
                .collect(),
        }
    }
    #[allow(dead_code)]
    pub fn to_csv(&self) -> String {
        self.xss
            .iter()
            .map(|line| line.join(","))
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[allow(dead_code)]
    pub fn from_md_table(md_table_str: &str) -> Result<Self> {
        let lines: Vec<&str> = md_table_str
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| x != &"") // filter out empty lines
            .collect();

        md_lines_are_valid(&lines)?;

        let mut xss = vec![md_line_to_row(lines[0])];
        xss.extend(lines[2..].iter().map(|x| md_line_to_row(*x)));
        Ok(Table { xss })
    }

    #[allow(dead_code)]
    pub fn to_md_table(&self) -> String {
        todo!()
    }
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    println!("Hello, world!");
}
