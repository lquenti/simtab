// TODO quote handling

#[derive(Debug)]
struct Table {
    xss: Vec<Row>,
}
type Row = Vec<Cell>;
type Cell = String;


impl Table {
    fn from_csv(csv_str: &str) -> Self {
        Self {
            xss: csv_str
                .split("\n")
                .map(|line| line.split(",").map(|x| x.to_owned()).collect::<Vec<Cell>>())
                .collect(),
        }
    }
    fn to_csv(self) -> String {
        self.xss
            .iter()
            .map(|line| line.join(","))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() {
    println!("Hello, world!");
}
