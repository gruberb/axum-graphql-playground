# Axum + GraphQL + RocksDB Playground
Experimenting with axum, GraphQL and RocksDB.

# Install

1. Clone repository

```bash
git clone git@github.com:gruberb/axum-graphql-playground.git
```

2. Have RocksDB installed

Example for macOS:

```bash
brew install rocksdb
```

Check if RocksDB is installed:

```bash
rocksdb_dump
```

# Run
```bash
cd axum-graphql-playground && cargo run
```

# Run with tracing
```bash
RUST_LOG=info cargo run
```

or

```bash
RUST_LOG=debug cargo run
```

# GraphQL Playground
After running the server, you can open the GraphQL playground under
```
http://localhost:8000.
```

Example queries:

```graphql
mutation {
  addBook(bookInput: { title: "Book Title", author: "Author Name" }) {
    id
    title
    author
  }
}

query {
  author(id:"3638fdcf-7f0e-45a7-8dbe-ca46f0280c9d") {
    books {
      title
    }
  }
}

query {
  author(id: "3638fdcf-7f0e-45a7-8dbe-ca46f0280c9d") {
    name
  }
}
```
