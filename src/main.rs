use std::time::{Duration, Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    sleep_time_as_micros: u64,
}

fn main() {
    let opt = Opt::from_args();
    loop {
        let expected_sleep_time = Duration::from_micros(opt.sleep_time_as_micros);
        let start = Instant::now();
        std::thread::sleep(expected_sleep_time);
        let actual_sleep_time = Instant::now() - start;
        println!(
            "Sleep overhead was {}us",
            (actual_sleep_time - expected_sleep_time).as_micros()
        );
    }
}
