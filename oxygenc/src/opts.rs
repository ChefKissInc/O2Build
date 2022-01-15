use clap::Parser;
use cranelift::prelude::*;

#[derive(Parser, Debug)]
#[clap(
    about = "Oxygen language compiler",
    version,
    author = "Copyright 2021 VisualDevelopment. All rights reserved."
)]
pub struct Options {
    #[clap(short = 'o', long, parse(from_os_str))]
    pub out: std::path::PathBuf,
    #[clap(parse(from_os_str))]
    pub src: std::path::PathBuf,
}

impl Options {
    pub fn to_target_isa(&self) -> Box<dyn isa::TargetIsa> {
        let mut builder = settings::builder();
        builder.set("is_pic", "true").unwrap();
        isa::lookup_by_name("x86_64-apple-darwin")
            .unwrap()
            .finish(settings::Flags::new(builder))
    }
}
