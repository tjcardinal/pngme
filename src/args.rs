use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub enum Args {
    Encode {
        filename: PathBuf,
        chunk_type: String,
        msg: String,
        output: Option<PathBuf>,
    },
    Decode {
        filename: PathBuf,
        chunk_type: String,
    },
    Remove {
        filename: PathBuf,
        chunk_type: String,
    },
    Print {
        filename: PathBuf,
    },
}
