use std::env;
use std::fs::File;
use std::path::Path;

use arrow::util::pretty::pretty_format_batches;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

fn exercise_1() {
    let source = String::from("compiler works");
    let message = source;

    println!("Rust data-movement exercise 1: {message}");
}

fn read_parquet(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;

    println!("Reading: {}", path.display());
    println!("Arrow schema:\n{}", builder.schema());

    let reader = builder.build()?;
    let mut row_count = 0;

    for batch_result in reader {
        let batch = batch_result?;
        row_count += batch.num_rows();

        println!("{}", pretty_format_batches(&[batch])?);
    }

    Ok(row_count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args().skip(1);

    match arguments.next() {
        None => exercise_1(),
        Some(path) => {
            let row_count = read_parquet(Path::new(&path))?;
            println!("Extracted {row_count} rows through Arrow.");
        }
    }

    Ok(())
}
