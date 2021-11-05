extern crate anyhow;
extern crate clap;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::task;

use clap::Parser;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "QingGo")]
struct Opts {
    file: Option<String>,
    #[clap(short, default_value = "3")]
    repeat_time: u64,
}

fn read_from_file_or_stdin<P>(
    file_path: &Option<P>,
) -> Result<io::Lines<io::BufReader<Box<dyn io::Read>>>, anyhow::Error>
where
    P: AsRef<Path>,
{
    let input: Box<dyn io::Read> = match *file_path {
        None => Box::new(io::stdin()),
        Some(ref file_path) => Box::new(fs::File::open(file_path)?),
    };
    Ok(io::BufReader::new(input).lines())
}

async fn measure_handshake_time(ip_post: &str, repeat_time: u64) -> f64 {
    let before = Instant::now();
    // 循环内 await 在同一个 task，串行。不同 task 之间则是并行
    for _ in 0..repeat_time {
        if let Err(_) = TcpStream::connect(ip_post).await {
            return 9999f64;
        }
    }
    return before.elapsed().as_secs_f64() / repeat_time as f64;
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();
    let input = read_from_file_or_stdin(&opts.file)?;
    let mut handles = Vec::new();
    for ip_port_result in input {
        handles.push(task::spawn(async move {
            ip_port_result.map(|ip_port| async move {
                let time = measure_handshake_time(&ip_port, opts.repeat_time).await;
                (ip_port, time)
            })
        }));
    }
    for handle in handles {
        let (ip_post, time) = handle.await??.await;
        println!("{} {:?}", ip_post, time);
    }
    Ok(())
}
