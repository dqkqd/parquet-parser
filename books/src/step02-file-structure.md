# Understand File Structure

Before implementing further, let's look at the parquet file structure and its file metadata. As
documented in the
[file metadata spec](https://parquet.apache.org/docs/file-format/metadata/#file-metadata), a parquet
file has multiple row groups, each row group has multiple columns, and each column has multiple
pages, which contain the actual data.

![parquet file structure, a file has many row groups, a row group has many columns, a column has many pages](images/parquet-file-structure.png)

To parse all columns, the parser needs to go down to the page level, extract the actual data and
concatenate them back as it goes to the upper levels.
