use clap::{CommandFactory, Parser, ValueEnum};
use std::{
    ffi::{c_char, c_int, c_uint, CString},
    path::PathBuf,
};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Format {
    /// Force ADF extraction
    ADF,
    /// Force ADZ extraction
    ADZ,
    /// Force DMS extraction
    DMS,
}

#[derive(Clone, Copy, Debug, Parser)]
#[group(multiple = false)]
struct FormatOpt {
    #[arg(long, value_enum)]
    format: Option<Format>,

    /// Force ADF extraction (if the filename ends in .adf then ADF will be assumed)
    #[arg(short = 'a')]
    force_adf: bool,

    /// Force ADZ extraction (if the filename ends in .adz or adf.gz then ADZ will be assumed)
    #[arg(short = 'z')]
    force_adz: bool,

    /// Force DMS extraction (if the filename ends in .dms then DMS format will be assumed)
    #[arg(short = 'd')]
    force_dms: bool,
}

impl FormatOpt {
    fn format(&self) -> Format {
        self.format.unwrap_or_else(|| {
            if self.force_adf {
                Format::ADF
            } else if self.force_adz {
                Format::ADZ
            } else if self.force_dms {
                Format::DMS
            } else {
                unreachable!()
            }
        })
    }
}

#[derive(Debug, Parser)]
struct Opts {
    /// Activate debugging output
    #[arg(short = 'D', long)]
    debug: bool,

    #[command(flatten)]
    format: Option<FormatOpt>,

    /// Set the starting sector of the extraction process. From 0 to 1760 (DD) or 3520 (HD)
    #[arg(short = 's', long, default_value_t = 0, value_parser = clap::value_parser!(u16).range(0..=3520))]
    start_sector: u16,

    /// Set the end sector of the extraction process. From 0 to 1760 (DD) or 3520 (HD)
    #[arg(short = 'e', long, default_value_t = 1760, value_parser = clap::value_parser!(u16).range(0..=3520))]
    end_sector: u16,

    /// Output file
    #[arg(short = 'o', long = "output")]
    output_file: Option<PathBuf>,

    /// adf/adz/dms filename
    input_file: PathBuf,
}

extern "C" {
    fn main_c(
        debug: c_int,
        format: c_int,
        start_sector: c_int,
        end_sector: c_uint,
        input: *const c_char,
        output: *const c_char,
    ) -> c_int;
}

fn main() {
    let opts = Opts::parse();

    if opts.start_sector > opts.end_sector {
        let mut cmd = Opts::command();
        cmd.error(
            clap::error::ErrorKind::ArgumentConflict,
            "End sector must be larger or equal to start sector.",
        )
        .exit();
    }

    let format = opts.format.map(|f| f.format());

    let c_format = match format {
        Some(Format::ADF) => 1,
        Some(Format::ADZ) => 2,
        Some(Format::DMS) => 3,
        None => 0,
    };

    let input = CString::new(opts.input_file.as_os_str().as_encoded_bytes()).unwrap();
    let output = opts
        .output_file
        .and_then(|p| CString::new(p.as_os_str().as_encoded_bytes()).ok());
    let result = unsafe {
        main_c(
            if opts.debug { 1 } else { 0 },
            c_format,
            opts.start_sector as c_int,
            opts.end_sector as c_uint,
            input.as_ptr(),
            output.map_or(std::ptr::null(), |p| p.as_ptr()),
        )
    };
    std::process::exit(result);
}
