# sleep-overhead
CLI tool to check your sleep() overhead

## Usage
```bash
git clone https://github.com/yisonPylkita/sleep-overhead
cd sleep-overhead
cargo run --release -- --sleep-time-as-micros 100
```

## Why would I use this tool?
Some virtualized systems (based on Windows VirtualBox especially) have a great inconsistency when it comes to timers. This tool allows you to measure that inconsistency.
On my bare-metal Linux I have these results
```
Sleep overhead was 55us
Sleep overhead was 54us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 54us
Sleep overhead was 54us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 55us
Sleep overhead was 54us
```

but on a Linux running in VirtualBox on Windows I have these results
```
TODO: fill this
```