// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    cli::base64::Base64SubCommand,
    process::b64::{process_decode, process_encode},
    process_csv, process_genpass, Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
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
            process_genpass(&opts)?;
        }

        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },
    }
    anyhow::Ok(())
    // println!("{:?}", opts);
}
