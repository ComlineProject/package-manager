// Relative Modules
pub mod gitlab;

// External Uses
use eyre::Result;


pub type ResourceServerFn = (&'static str, fn() -> Result<()>);

pub const RESOURCE_SERVERS: &'static [ResourceServerFn] = &[
    ("gitlab", gitlab::authenticate),
];

/*
pub fn resource_servers() -> &'static [ResourceServerFn] {

}
*/
