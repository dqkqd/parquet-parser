# Nulls

In the following steps, we will handle
[columns containing nulls](https://parquet.apache.org/docs/file-format/nulls/). There are two
important notes:

- Null values aren't encoded in the data page.
- Null information are stored in definition levels.

![nulls in data pages and definition levels](images/nulls-data-pages-and-definition-levels.png)
