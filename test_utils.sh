# Loop over stdins directory and copy each stdin file
# Copy the range-elf to the s3 bucket as program.bin
for stdin_file in stdins/stdin-*.bin; do
    if [ -f "$stdin_file" ]; then
        start=$(echo $stdin_file | grep -oP '(?<=stdin-)\d+(?=-)')
        end=$(echo $stdin_file | grep -oP '(?<=-)\d+(?=\.bin)')

        aws s3 cp "$stdin_file" "s3://sp1-testing-suite/op-succinct-op-sepolia-$start-$end/stdin.bin"
        aws s3 cp elf/range-elf "s3://sp1-testing-suite/op-succinct-op-sepolia-$start-$end/program.bin"
    fi
done

