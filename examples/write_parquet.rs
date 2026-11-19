use std::fs;
use std::fs::File;
use std::sync::Arc;

use arrow::array::{Int32Array, RecordBatch, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use parquet::arrow::ArrowWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("data")?;

    let schema = Arc::new(Schema::new(vec![
        Field::new("order_id", DataType::Int32, false),
        Field::new("customer", DataType::Utf8, false),
    ]));

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int32Array::from(vec![101, 102, 103])),
            Arc::new(StringArray::from(vec!["Ada", "Lin", "Grace"])),
        ],
    )?;

    let file = File::create("data/orders.parquet")?;
    let mut writer = ArrowWriter::try_new(file, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;

    println!("Created data/orders.parquet");
    Ok(())
}
