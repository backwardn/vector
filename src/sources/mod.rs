use futures::Future;

pub mod docker;
pub mod file;
pub mod journald;
#[cfg(feature = "rdkafka")]
pub mod kafka;
pub mod kubernetes;
pub mod socket;
pub mod prometheus;
pub mod splunk_hec;
pub mod statsd;
pub mod stdin;
pub mod syslog;
mod util;
pub mod vector;

pub type Source = Box<dyn Future<Item = (), Error = ()> + Send>;
