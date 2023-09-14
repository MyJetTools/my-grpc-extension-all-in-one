mod grpc_channel;
mod grpc_client_interceptor;
pub mod grpc_server;
mod grpc_server_telemetry_context;

pub use grpc_server_telemetry_context::*;
pub mod read_grpc_stream;
pub use grpc_channel::*;
pub use grpc_client_interceptor::*;


#[cfg(feature = "grpc-client")]
extern crate my_grpc_client_macros;
#[cfg(feature = "grpc-client")]
pub use my_grpc_client_macros::*;


#[cfg(feature = "grpc-server")]
extern crate my_grpc_server_macros;
#[cfg(feature = "grpc-server")]
pub use my_grpc_server_macros::*;