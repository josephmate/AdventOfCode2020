$env:RUST_BACKTRACE=1

cargo clippy
Get-Content sample_input_01.txt | cargo run > actual_sample_output_01.txt
