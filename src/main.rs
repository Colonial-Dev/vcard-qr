mod cli;
mod vcard;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use dialoguer::{Confirm, Editor, Input};

use crate::cli::*;
use crate::vcard::VCard;

const FORMATTED_NAME: &str = "FN";
const EMAIL: &str = "EMAIL";
const TELEPHONE: &str = "TEL";
const ADDRESS: &str = "ADR";
const WEBSITE: &str = "URL";
const NOTE: &str = "NOTE";

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "Building new vCard... [ Format: {:?} // ECC Level: {:?} // Size: {}px ]\n",
        &cli.format, &cli.error_correction, &cli.size
    );

    if let Some(s) = &cli.from {
        let vcard = fs::read_to_string(s)?;
        let filename = cli.output_name.clone().unwrap_or(s.trim_end_matches(".vcf").to_string());
      
        write_vcard_qr(
            vcard,
            &cli,
            &filename
        )?;
    } else {
        let (vcard, name) = build_vcard()?;
        let mut filename = cli.output_name.clone().unwrap_or("vcard".to_string());
        if cli.prefix_name {
            filename = format!("{name}-{filename}");
        }

        write_vcard(
            vcard.as_bytes(),
            &filename
        )?;

        write_vcard_qr(
            vcard,
            &cli,
            &filename
        )?;
    }
  
    Ok(())
}

fn build_vcard() -> Result<(String, String)> {
    let mut vcard = VCard::new();

    let name = query_input("Your name (required)", false)?;

    if query_bool ("Do you want to specify name components (e.g. first/middle/last)?", false)? {
        vcard.push("N", query_name_components()?)
    }

    let email = query_input("Contact email (recommended)", true)?;
    let phone = query_input("Contact phone (recommended)", true)?;
    let website = query_input("Your website (optional)", true)?;

    vcard.push(FORMATTED_NAME, &name);
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
            vcard.optional_push(NOTE, note.replace('\n', "\\n"))
        } else {
            println!("Skipped adding a note.")
        }
    }

    Ok((
        vcard.finalize(),
        name
    ))
}

fn write_vcard(vcard: &[u8], name: &str) -> Result<()> {
    let vcf = format!("{name}.vcf");
    let bytes = File::create(&vcf)
        .map(|mut f| f.write(vcard))?;

    println!("vCard written to \"{}\" ({} bytes)", vcf, bytes.unwrap());
    Ok(())
}

fn write_vcard_qr(vcard: String, config: &Cli, name: &str) -> Result<()> {
    let mut path = PathBuf::from(name);

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


        let street_addr = match extended_addr.len() {
            0 => street_addr,
            _ => format!("{street_addr}\\n{extended_addr}"),
        };

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

fn query_name_components() -> Result<String> {
    println!("Note: all name components can have multiple comma-separated values.");

    let family_name = query_input("Family name(s)", true)?;
    let given_name  = query_input("Given name(s)", true)?;
    let middle_name = query_input("Middle name(s)", true)?;
    let honor_pre   = query_input("Honorific prefix(es)", true)?;
    let honor_suf   = query_input("Honorific suffix(es)", true)?;

    Ok(
        format!("{family_name};{given_name};{middle_name};{honor_pre};{honor_suf}")
    )
}