# Introduction

This book is part of my parquet file format learning journey. It shows you how to implement a
parquet parser from scratch. The goal of this book is to understand the internal file format by
actually parsing it. This, however, doesn't provide you with any information about how to use
parquet, or how to query it efficiently.

The book itself is written as a series of exercises, each step contains test cases to verify the
implementation. At the end, the parser will be able to read parquet files containing primitive
types, basic encodings, compressions, and nulls.

**The parser is written in Rust and readers are expected to implement all the steps themselves to
have a working parser.**
