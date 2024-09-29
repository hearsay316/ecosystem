###
链接的docker :

docker run -d -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

检查
cargo fmt -- --check

cargo clippy --all-targets --all-features --tests --benches -- -D warnings

修复
cargo fmt 