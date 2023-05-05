use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use proc_macro2::TokenStream;
use crate::deserialize_json_array_as_stream::iter_json_array;
use crate::error::Error;
use crate::node_types::NodeType;

pub fn generate(mut path: PathBuf) -> Result<TokenStream, Error> {
    if let Ok(cargo_manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        path = PathBuf::from(cargo_manifest).join(path);
    }
    let node_types = iter_json_array::<NodeType, _>(BufReader::new(File::open(path)?));
    node_types.map(|node_type| node_type.map(|x| x.print()).map_err(Error::from)).collect()
}