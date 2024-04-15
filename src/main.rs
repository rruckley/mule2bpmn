
use std::fs;

use log::{info,error};
use xmltree::{Element, XMLNode};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short,long)]
    filename: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    info!("Starting {pkg} v{ver}");

    let filename = args.filename;

    info!("Reading file : {filename}");

    let file_contents = fs::read_to_string(filename)
        .expect("Could not open file");

    let xml = Element::parse(file_contents.as_bytes());

    // Ignore error case for now.
    let root_xml = xml.unwrap();

    // Iterate through children, pullng out flow nodes
    root_xml.children.iter().for_each(|n| {
        match n {
            XMLNode::Element(e) => {
                let flow_name = e.attributes.get("name").cloned().unwrap_or_default();
                info!("Found node: {} with name {}",e.name,flow_name);
            },
            _ => {
                // Don't care about these
            }
        }
    })
}
