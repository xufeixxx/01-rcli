// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

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
    }
    anyhow::Ok(())
    // println!("{:?}", opts);
}
