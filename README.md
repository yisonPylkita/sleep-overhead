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
Sleep overhead was 37us
Sleep overhead was 955us
Sleep overhead was 53us
Sleep overhead was 219us
Sleep overhead was 139us
Sleep overhead was 178us
Sleep overhead was 198us
Sleep overhead was 330us
Sleep overhead was 116us
Sleep overhead was 141us
Sleep overhead was 241us
Sleep overhead was 138us
Sleep overhead was 208us
Sleep overhead was 381us
Sleep overhead was 865us
Sleep overhead was 1380us
Sleep overhead was 75us
Sleep overhead was 43us
Sleep overhead was 997us
Sleep overhead was 79us
Sleep overhead was 573us
Sleep overhead was 193us
Sleep overhead was 209us
Sleep overhead was 175us
Sleep overhead was 102us
Sleep overhead was 1314us
Sleep overhead was 520us
Sleep overhead was 216us
Sleep overhead was 191us
Sleep overhead was 33us
Sleep overhead was 161us
Sleep overhead was 662us
Sleep overhead was 1303us
```


// TODO: add line chart
