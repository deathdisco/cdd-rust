use crate::util;
use cdd::*;
use std::path::PathBuf;

pub fn write_model(model: Model, file: PathBuf) -> Result<(), failure::Error> {
    util::write_file(file, "struct Guppy {};")
}
