use std::fs;
// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    cli::{
        base64::Base64SubCommand,
        http::HttpSubCommand,
        text::{TextSignFormat, TextSubCommand},
    },
    process::{
        b64::{process_decode, process_encode},
        http_serve::process_http_serve,
        text::{process_generate_key, process_text_sign, process_text_verify},
    },
    process_csv, process_genpass, Opts, SubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output: String = if let Some(op_str) = opts.output {
                op_str.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }

        SubCommand::GenPass(opts) => {
            let password_string = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password_string);
            let estimate = zxcvbn(&password_string, &[])?;
            // output password strength in stderr
            eprintln!("Password strength {}", estimate.score());
        }

        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },

        SubCommand::Text(opts) => match opts {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::GenerateKey(opts) => {
                let key = process_generate_key(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?; // sign key
                        fs::write(name.join("ed25519.pk"), &key[1])?; // public key
                    }
                }
            }
        },

        SubCommand::Http(opts) => match opts {
            HttpSubCommand::Serve(opts) => {
                println!("{:?}", opts);
                process_http_serve(opts.path, opts.port).await?;
            }
        },
    }
    anyhow::Ok(())
    // println!("{:?}", opts);
}
