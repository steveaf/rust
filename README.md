# Rust Data-Movement Tutorial

This tutorial starts with the smallest possible Rust exercise, then moves to extracting Parquet data through Apache Arrow. It also maps the Parquet concepts to Delta Lake and Apache Iceberg.

## Prerequisites

1. Install Rust and Cargo:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. Verify the toolchain:

   ```bash
   rustc --version
   cargo --version
   ```

3. Open this directory in VS Code and install the `rust-analyzer` extension.

## Exercise 1: prove that the compiler works

The `src/main.rs` program contains `exercise_1()`. It demonstrates the most important first data-movement concept: Rust transfers a value by binding it to a name.

Run:

```bash
cargo run
```

Expected output:

```text
Rust data-movement exercise 1: compiler works
```

The key lines are:

```rust
let source = String::from("compiler works");
let message = source;
```

Here, `source` owns the `String`. The second line moves ownership to `message`. Rust does not automatically make a deep copy. After the move, `source` is no longer available.

Try changing `message` into a `&str` or cloning `source`:

```rust
let source = String::from("compiler works");
let message = source.clone();
println!("{} and {}", source, message);
```

Use borrowing with `&` when a function only needs to inspect data:

```rust
fn print_message(message: &String) {
    println!("{message}");
}
```

Use `clone()` when independent ownership is genuinely needed. Borrowing is usually cheaper.

## Core mental model

In this tutorial, data moves through three kinds of boundaries:

1. **Storage boundary:** bytes enter from disk or object storage.
2. **Format boundary:** Parquet decode logic converts those bytes into Arrow arrays and record batches.
3. **Application boundary:** Rust moves or borrows those batches to processing and extraction code.

Apache Arrow is the in-memory data format. Apache Parquet is primarily a storage/serialization format. Arrow and Parquet do not replace each other: Parquet stores the data, and Arrow represents the data in memory.

## Exercise 2: create an Arrow table and write Parquet

Generate a small Parquet file:

```bash
cargo run --example write_parquet
```

This example creates `data/orders.parquet` with an Arrow `Schema` and a `RecordBatch` containing `order_id` and `customer` columns.

A `RecordBatch` is a rectangular chunk of data: it has one schema and a set of columns represented as Arrow arrays. A reader can return multiple batches, so the application must process each batch separately.

## Exercise 3: extract the Parquet file through Arrow

Run:

```bash
cargo run -- data/orders.parquet
```

The program opens the file, constructs a `ParquetRecordBatchReaderBuilder`, prints the Arrow schema, iterates over record batches, and prints one batch at a time.

The central loop is:

```rust
for batch_result in reader {
    let batch = batch_result?;
    println!("{} rows", batch.num_rows());
}
```

The `?` operator returns any error from the function. The `Box<dyn std::error::Error>` return type lets file, Arrow, and Parquet errors use one boundary while you learn.

## Exercise 4: extract selected columns and use the data

Avoid reading every column from an expensive Parquet file. Configure the builder's projection first:

```rust
let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;

let indices = builder
    .schema()
    .fields()
    .iter()
    .enumerate()
    .filter(|(_, field)| field.name() == "customer")
    .map(|(index, _)| index)
    .collect::<Vec<_>>();

let reader = builder
    .with_projection(indices)
    .build()?;
```

Projection reduces which physical columns are read. Predicate pushdown is different: it reduces which row groups or rows need to be decoded. For remote object storage, combine projection, predicate pushdown, partition pruning, and batch sizing.

## Delta Lake

A Delta table is a table-format transaction log plus data files, usually Parquet. The Rust ecosystem has Delta implementations such as `deltalake` and integrations with Arrow and object stores. A typical flow is:

```text
Catalog/table path
  -> read Delta log and resolve a snapshot
  -> select active Parquet files
  -> apply partition and file pruning
  -> read files with Arrow/Parquet
  -> process RecordBatch values
```

Do not read a Delta directory as if every file inside one snapshot is live data. Deleted files and old versions remain in storage. Resolve the Delta log, obtain the active file set, then read those files through Arrow.

A useful next dependency is:

```toml
object_store = "..."
deltalake = { version = "...", features = ["arrow"] }
```

Use the version and feature names supported by the exact crate release you choose. Then continue from its `Table::load` or `DeltaTable` API to obtain Arrow record batches.

## Apache Iceberg

Apache Iceberg is another table format with a metadata catalog, snapshots, manifests, and data files. A typical flow is:

```text
Catalog + table identifier
  -> load table metadata
  -> choose a snapshot
  -> plan manifests and data files
  -> prune files and partitions
  -> read active Parquet files with Arrow
  -> process RecordBatch values
```

Rust implementations include `iceberg` and catalog/object-store integrations. Because their APIs change across releases, select a compatible Arrow/Parquet/object-store version set before adding a catalog dependency.

The extraction loop itself remains Arrow-oriented:

```rust
for batch_result in reader {
    let batch = batch_result?;
    // Filter, transform, aggregate, or move batch to another Arrow-aware system.
}
```

## Data-movement checklist

- Borrow when a function only inspects data; use `&Batch` or `&[RecordBatch]`.
- Use `.clone()` for an independent copy; do not use it as a habit for every ownership issue.
- Streams of record batches avoid loading an entire table into memory.
- Reuse an Arrow schema and batch size where possible.
- Select required columns before reading remote files.
- Use partition pruning, snapshot/version selection, and predicate pushdown.
- Preserve errors with `?` instead of silently converting failures to `None`.
- Keep table-format state out of low-level Parquet functions.

## Suggested next exercises

1. Add a projection to the existing reader and print only `customer`.
2. Count rows and distinct customer names across multiple batches.
3. Add a `--batch-size` argument while reading Parquet.
4. Implement a local Delta table and inspect only the latest snapshot's active files.
5. Add an Iceberg catalog, plan a snapshot, and feed resulting Parquet files into this reader.
6. Export the final Arrow batches to CSV or stream them to another Arrow-compatible service.
