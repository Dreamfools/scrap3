export CARGO_PROFILE_BENCH_DEBUG=true

cargo flamegraph -F 10000 -p match3 --bench match3 -- --bench