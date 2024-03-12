use nih_plug::prelude::*;

use comprs::CompressorPlugin;

fn main() {
    nih_export_standalone::<CompressorPlugin>();
}
