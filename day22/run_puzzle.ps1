$env:RUST_BACKTRACE=1

cargo clippy
Get-Content puzzle_input.txt | cargo run > puzzle_output.txt
