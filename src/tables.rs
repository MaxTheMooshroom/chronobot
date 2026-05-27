
use anyhow::Result;
use polars::prelude::*;

use std::fs::File;
use std::path::Path;
use std::sync::LazyLock;

use crate::util::path_in_cwd_or_exedir;

static CSV_ROOT: LazyLock<&Path> = LazyLock::new(|| "tables/csv".as_ref());

pub static KINDS: LazyLock<DataFrame>
    = LazyLock::new(|| read_table_required("kinds.csv"));

pub static STAT_TYPES: LazyLock<Box<[String]>>
    = LazyLock::new(|| column_a_where_column_b_is(&KINDS, "name", "value", "stat"));

fn read_table<P: AsRef<Path>>(path: P) -> Result<DataFrame> {
    let full_path = path_in_cwd_or_exedir(CSV_ROOT.join(path.as_ref()))?;

    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(full_path))?
        .finish()
        .map_err(|e| e.into())
}

#[inline]
fn read_table_required<P: AsRef<Path>>(path: P) -> DataFrame {
    read_table(path).expect("required csv was not able to be read into a dataframe")
}

fn column_a_where_column_b_is(df: &'static DataFrame, b: &str, a: &str, value: &str) -> Box<[String]>
{
    let b_col = df.column(b)
        .unwrap_or_else(|_| panic!("Could not find column {b} in dataframe"));

    let filter = df
        .column(a).unwrap_or_else(|_| panic!("Could not find column {a} in dataframe"))
        .str().unwrap_or_else(|_| panic!("could not cast a column {a} to dtype str"))
        .equal(value);

    b_col.filter(&filter).unwrap()
        .str().unwrap()
        .into_iter()
        .map(Option::unwrap)
        .map(str::to_string)
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

