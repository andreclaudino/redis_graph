//! redis_graph provides a small trait with two extension functions for the
//! [redis](https://docs.rs/redis/) crate to allow working with redis graph
//! data types that can be installed as a [redis module](https://oss.redislabs.com/redisgraph).
//! Redis graph operation are only using two top level Redis commands
//! (one for read/write operations and one for read-only operations), so
//! this crate only adds two functions to the redis commands.
//! The Graph commands are available in synchronous and asynchronous versions.
//!
//! The crate is called `redis-graph` and you can depend on it via cargo. You will
//! also need redis in your dependencies.
//!
//! ```ini
//! [dependencies]
//! redis = "0.19.0"
//! redis-graph = "*"
//! ```
//!
//! Or via git:
//!
//! ```ini
//! [dependencies.redis-graph]
//! git = "https://github.com/tompro/redis_graph.git"
//! branch = "main"
//! ```
//!
//! With async feature inherited from the [redis](https://docs.rs/redis)
//! crate (either: 'async-std-comp' or 'tokio-comp):
//!
//! ```ini
//! [dependencies]
//! redis = "0.19.0"
//! redis-graph = { version = "0.3.0", features = ['tokio-comp'] }
//! ```
//!
//! # Synchronous usage
//!
//! To enable the redis graph commands you simply load the trait
//! redis_graph::GraphCommands into scope. The redis graph
//! commands will then be available on your redis connection.
//! To also have access to the value extractor traits simply import
//! the whole crate redis_graph::*.
//!
//!  
//! ```rust,no_run
//! # fn run() -> redis::RedisResult<()> {
//! use redis::Commands;
//! use redis_graph::*;
//!
//! let client = redis::Client::open("redis://127.0.0.1/")?;
//! let mut con = client.get_connection()?;
//!
//! let _:GraphResultSet = con.graph_query(
//!     "my_graph",
//!     "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
//! )?;
//!
//! let _:GraphResultSet = con.graph_ro_query(
//!     "my_graph",
//!     "MATCH (rider:Rider)-[:rides]->(:Team {name:'Yamaha'}) RETURN rider"
//! )?;
//! # Ok(()) }
//! ```
//!
//!
//! # Asynchronous usage
//!
//! To enable the redis graph async commands you simply load the
//! redis_graph::AsyncGraphCommands into the scope. To also have access
//! to the value extractor traits simply import the whole crate redis_graph::*.
//!
//! ```rust,no_run
//! # #[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
//! # async fn run() -> redis::RedisResult<()> {
//! use redis::AsyncCommands;
//! use redis_graph::*;
//!
//! let client = redis::Client::open("redis://127.0.0.1/")?;
//! let mut con = client.get_async_connection().await?;
//!
//! let _:GraphResultSet = con.graph_query(
//!     "my_graph",
//!     "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
//! ).await?;
//!
//! let _:GraphResultSet = con.graph_ro_query(
//!     "my_graph",
//!     "MATCH (rider:Rider)-[:rides]->(:Team {name:'Yamaha'}) RETURN rider"
//! ).await?;
//! # Ok(()) }
//! ```
//!
//! # Commands
//!
//! The following examples work with the synchronous and asynchronous API. For
//! simplicity all examples will use the synchronous API. To use them async simply
//! run them within an async function and append the .await after the command call.
//!
//! ## GRAPH.QUERY and GRAPH.RO_QUERY
//! The query command (and read-only alternative) is the only command required for
//! all graph operations. It will produce a GraphResultSet that contains the
//! results of the operation. As queries are very flexible a lot of different
//! data stuctures can be contained in a GraphResultSet.
//!
//! ```rust,no_run
//! # fn run() -> redis::RedisResult<()> {
//! use redis::Commands;
//! use redis_graph::*;
//!
//! let client = redis::Client::open("redis://127.0.0.1/")?;
//! let mut con = client.get_connection()?;
//!
//! /// A create query returns metadata as a list of strings
//! let r:GraphResultSet = con.graph_query(
//!     "my_graph",
//!     "CREATE (:Rider {name:'Valentino Rossi'})-[:rides]->(:Team {name:'Yamaha'})"
//! )?;
//! assert!(!r.metadata.is_empty());
//!
//!
//! /// This read-only query will return nodes and scalars in the result
//! let riders = con.graph_ro_query(
//!     "my_graph",
//!     "MATCH (rider:Rider)-[:rides]->(team:Team)
//!        WHERE team.name = 'Yamaha'
//!        RETURN rider, team.name"
//! )?;
//!
//!
//! /// Data contains a vec of GraphResult with one entry per query match.
//! assert!(riders.data.len() > 0);
//!
//!
//! /// A GraphResult is indexed with the identifiers used in the RETURN
//! /// clause of the query. A result has some convenience functions to
//! /// extract GraphValues (Scalar|Node|Relation) into rust types.
//! let entry = riders.data.get(0).unwrap();
//! let rider:Option<&NodeValue> = entry.get_node("rider");
//! let team_name:Option<String> = entry.get_scalar("team.name");
//!
//!
//! /// Node and Relation values can contain properties for which there are
//! /// value extractors as well.
//! let rider_name:Option<String> = rider.unwrap().get_property_option("name");
//!
//! # Ok(()) }
#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
pub use crate::async_commands::AsyncGraphCommands;
pub use crate::commands::GraphCommands;
pub use crate::types::*;

#[cfg(any(feature = "tokio-comp", feature = "async-std-comp"))]
mod async_commands;
mod commands;
mod types;
