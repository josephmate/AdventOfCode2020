$env:RUST_BACKTRACE=1


Get-Content sample_input_02.txt | cargo run false true false > actual_sample_output_02.txt
