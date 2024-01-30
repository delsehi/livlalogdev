use livla_log::get_json_from_query;
use std::fs::File;
use std::io::{BufReader, Read, Write};

#[tokio::main]
async fn main() {
    let input_file = File::open("./test.txt.bak").unwrap();
    let mut input = String::new();
    let mut buf_reader = BufReader::new(input_file);
    buf_reader.read_to_string(&mut input).unwrap();

    let input_sql_file = File::open("./test.sql").unwrap();
    let mut sql_input = String::new();
    let mut sql_buf_reader = BufReader::new(input_sql_file);
    sql_buf_reader.read_to_string(&mut sql_input).unwrap();

    let json = get_json_from_query(&input, &sql_input).await;

    let mut file = File::create("./result.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("{json}");
}
