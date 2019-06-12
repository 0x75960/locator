extern crate structopt;

use std::io::prelude::*;
use structopt::StructOpt;

type GenericResult<T> = Result<T, failure::Error>;

/// locate the service that has specified sample
#[derive(StructOpt)]
#[structopt(name = "locator")]
struct Opt {
    /// check apikeys to be used
    #[structopt(short = "c", long = "check-keys")]
    check_keys: bool,

    /// how many seconds wait between requests
    #[structopt(short = "d", long = "delay", default_value = "15")]
    delay: u64,
}

fn check_keys() -> GenericResult<()> {
    let mut errors: Vec<String> = Vec::new();

    let targets = vec!["VTAPIKEY", "REVIT_APIKEY", "MALSHARE_APIKEY", "OTX_APIKEY"];

    targets.into_iter().for_each(|target| {
        if let None = std::env::var_os(target) {
            errors.push(target.to_owned());
        }
    });

    if errors.is_empty() {
        Ok(())
    } else {
        Err(failure::err_msg(format!(
            "these envvar not found: [{}]",
            errors.join(",")
        )))
    }
}

fn proc(cli: &festum::Client, hash: impl AsRef<str>) -> String {
    let hash = hash.as_ref();
    let res = cli.query(hash);
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        hash, res.virustotal, res.alienvault, res.virusbay, res.reverseit, res.malshare
    )
}

fn main() -> GenericResult<()> {
    let o = Opt::from_args();
    if o.check_keys {
        return match check_keys() {
            Ok(_) => {
                println!("ok");
                Ok(())
            }
            Err(e) => Err(e),
        };
    }

    println!("hash\tvirustotal\talienvault otx\tvirusbay\treverse.it\tmalshare");

    let cli = festum::Client::default();

    std::io::stdin()
        .lock()
        .lines()
        .inspect(|_| std::thread::sleep(std::time::Duration::from_secs(o.delay)))
        .for_each(|x| {
            println!("{}", proc(&cli, x.unwrap()));
        });

    Ok(())
}
