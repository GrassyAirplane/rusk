# rusk
Rust implementation of spark


```
rusk-engine/
├── Cargo.toml                 # Workspace configuration
├── README.md
├── .gitignore
├── rusk-core/                # Core distributed computing engine
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Main module exports
│       ├── error.rs          # Error types and handling
│       ├── context.rs        # RuskContext - main entry point
│       ├── rdd.rs            # RDD abstraction and operations
│       ├── partition.rs      # Partition trait and implementations
│       ├── task.rs           # Task execution primitives
│       ├── scheduler.rs      # DAG scheduler and optimization
│       ├── executor.rs       # Task execution management
│       ├── shuffle.rs        # Data shuffling for wide dependencies
│       └── storage.rs        # Caching and storage management
├── rusk-sql/                 # SQL query engine (DataFusion integration)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── dataframe.rs      # DataFrame API
│       ├── sql_context.rs    # SQL execution context
│       └── catalyst.rs       # Query optimization
├── rusk-connectors/          # Data source connectors
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── jdbc.rs           # JDBC connector
│       ├── mongodb.rs        # MongoDB connector  
│       ├── s3.rs             # S3 object store connector
│       └── parquet_connector.rs # Parquet file format
├── rusk-runtime/             # Cluster management and networking
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── cluster.rs        # Cluster manager
│       ├── driver.rs         # Driver program
│       ├── worker.rs         # Worker node
│       └── network.rs        # Inter-node communication
├── rusk-cli/                 # Command-line interface
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── examples/                 # Usage examples
    ├── Cargo.toml
    └── src/
        ├── basic_etl.rs      # JDBC -> Parquet -> S3 example
        ├── mongodb_to_s3.rs  # MongoDB -> S3 pipeline
        └── sql_queries.rs    # SQL processing examples
```