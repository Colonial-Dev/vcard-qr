use clap::{Parser, ValueEnum};
use qrcode_generator::QrCodeEcc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The desired name of the output file, sans extension.
    #[arg(short, long, default_value = "vcard")]
    pub output_name: String,
    /// The desired output format of the QR code.
    #[arg(short, long, value_enum, default_value_t=OutputFormat::Svg)]
    pub format: OutputFormat,
    /// The desired error correction level.
    /// Higher levels generate larger QR codes, but make it more likely
    /// the code will remain readable if it is damaged.
    #[arg(short, long, value_enum, default_value_t=ErrorCorrection::Medium)]
    pub error_correction: ErrorCorrection,
    /// The size of the output image, in pixels.
    #[arg(short, long, default_value = "1024")]
    pub size: usize,
    /// vcf File to read from
    #[arg(long)]
    pub from: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ErrorCorrection {
    Low,
    Medium,
    High,
    Max,
}

#[allow(clippy::from_over_into)]
impl Into<QrCodeEcc> for ErrorCorrection {
    fn into(self) -> QrCodeEcc {
        match self {
            ErrorCorrection::Low => QrCodeEcc::Low,
            ErrorCorrection::Medium => QrCodeEcc::Medium,
            ErrorCorrection::High => QrCodeEcc::Quartile,
            ErrorCorrection::Max => QrCodeEcc::High,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Png,
    Svg,
}