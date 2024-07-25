#![warn(clippy::perf, clippy::style, warnings)]

mod cli;
mod vcard;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::cli::*;
use crate::vcard::VCard;
use anyhow::Result;
use clap::Parser;
use dialoguer::{Confirm, Editor, Input};

const FORMATTED_NAME: &str = "FN";
const EMAIL: &str = "EMAIL";
const TELEPHONE: &str = "TEL";
const ADDRESS: &str = "ADR";
const WEBSITE: &str = "URL";
const NOTE: &str = "NOTE";

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "Building new VCard... [ Format: {:?} // ECC Level: {:?} // Size: {}px ]\n",
        &cli.format, &cli.error_correction, &cli.size
    );

    let vcard = build_vcard()?;
    write_vcard(vcard.as_bytes(), cli.output_name.clone())?;
    write_vcard_qr(vcard, cli)?;
    Ok(())
}

fn build_vcard() -> Result<String> {
    let mut vcard = VCard::new();

    let name = query_input("Your name (required)", false)?;
    let email = query_input("Contact email (recommended)", true)?;
    let phone = query_input("Contact phone (recommended)", true)?;
    let website = query_input("Your website (optional)", true)?;

    vcard.push(FORMATTED_NAME, name);
    vcard.optional_push(EMAIL, email);
    vcard.optional_push(TELEPHONE, phone);
    vcard.optional_push(WEBSITE, website);

    if query_bool("Do you want to add addresses?", false)? {
        for address in query_addresses()? {
            vcard.push_explicit(&address);
        }
    }

    if query_bool("Do you want to add a note?", false)? {
        if let Some(note) = Editor::new().edit("")? {
            vcard.optional_push(NOTE, note.replace("\n", "\\n"))
        } else {
            println!("Skipped adding a note.")
        }
    }

    Ok(vcard.finalize())
}

fn write_vcard(vcard: &[u8], filename: String) -> Result<()> {
    let mut vcf = PathBuf::from(filename);
    vcf.set_extension("vcf");
    let bytes = File::create(vcf.clone()).map(|mut f| f.write(vcard))?;
    println!("vCard written to \"{}\" ({} bytes)", vcf.to_string_lossy(), bytes.unwrap());
    Ok(())
}

fn write_vcard_qr(vcard: String, config: Cli) -> Result<()> {
    use std::path::PathBuf;

    let mut path = PathBuf::from(config.output_name);

    match config.format {
        OutputFormat::Png => {
            path.set_extension("png");
            qrcode_generator::to_png_to_file(
                vcard,
                config.error_correction.into(),
                config.size,
                &path,
            )?
        }
        OutputFormat::Svg => {
            path.set_extension("svg");
            qrcode_generator::to_svg_to_file(
                vcard,
                config.error_correction.into(),
                config.size,
                None::<&str>,
                &path,
            )?
        }
    }

    println!("QR Code written to \"{}\"", path.to_string_lossy());
    Ok(())
}

/// Convenience function that wraps [`dialoguer::Input`] for prompting a string from the user.
fn query_input(prompt: &str, optional: bool) -> Result<String, std::io::Error> {
    Input::new()
        .with_prompt(prompt)
        .allow_empty(optional)
        .interact_text()
}

/// Convenience function that wraps [`dialoguer::Confirm`] for prompting a y/n decision from the user.
fn query_bool(prompt: &str, default: bool) -> Result<bool, std::io::Error> {
    Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()
}

/// Collects an arbitrary number of addresses from the user, in a loop.
fn query_addresses() -> Result<Vec<String>> {
    let mut addresses: Vec<String> = Vec::new();

    loop {
        let street_addr = query_input("Street address (required)", false)?;
        let extended_addr =
            query_input("Extended address (e.g. apartment number, optional)", true)?;
        let city = query_input("Municipality (required)", false)?;
        let state = query_input("State/province (required)", false)?;
        let zip = query_input("ZIP/postal code (required)", false)?;
        let country = query_input("Country (optional)", true)?;
        let addr_type = query_input("Address type (e.g. home, optional)", true)?;

        let street_addr = format!("{street_addr},{extended_addr}");

        let property = match addr_type.is_empty() {
            false => format!("{ADDRESS};TYPE={addr_type}"),
            true => ADDRESS.to_string(),
        };

        addresses.push(format!(
            "{property}:;;{street_addr};{city};{state};{zip};{country}"
        ));

        if query_bool("Do you want to add another address?", false)? {
            continue;
        }

        break;
    }

    Ok(addresses)
}
