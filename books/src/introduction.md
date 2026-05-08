# Introduction

This book is part of my [parquet](https://parquet.apache.org/) learning journey. It shows you how to
implement a parquet parser from scratch. The goal of this book is to understand the internals of the
parquet file format by actually writing a parser. It does not, however, cover how to to use parquet,
or how to query it efficiently.

The book itself is written as a series of exercises; each step contains test cases to verify the
implementation. At the end, the parser will be able to read parquet files with primitive types,
basic encodings, compression, and nulls.

**The parser is written in Rust and readers are expected to implement all the steps themselves to
have a working parser.**
