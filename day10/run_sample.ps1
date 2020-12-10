$env:RUST_BACKTRACE=1


Get-Content sample_input_01.txt | cargo run 5 > actual_sample_output_01.txt
