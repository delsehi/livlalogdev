mod parser;
mod query_engine;
use datafusion::arrow::json::ArrayWriter;
use datafusion::arrow::record_batch::RecordBatch;
pub use parser::input_to_record_batch;
use query_engine::get_context;

pub async fn get_json_from_query(livla_logs: &str, sql_query: &str) -> (String, Vec<String>) {
    let batch = input_to_record_batch(&livla_logs).unwrap();
    let ctx = get_context();
    ctx.register_batch("lifts", batch).unwrap();
    let df = ctx.sql(&sql_query).await.unwrap();
    let schema = df
        .schema()
        .fields()
        .iter()
        .map(|e| e.name().to_owned())
        .collect::<Vec<String>>();

    let record_batch = df.collect().await.unwrap();
    let record_batch: Vec<&RecordBatch> = record_batch.iter().collect();
    let buffer = Vec::new();
    let mut writer = ArrayWriter::new(buffer);

    writer.write_batches(&record_batch).unwrap();
    writer.finish().unwrap();
    let buf = writer.into_inner();
    let json_string = String::from_utf8(buf).unwrap();
    (json_string, schema)
}
