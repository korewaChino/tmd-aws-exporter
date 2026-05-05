# TMD AWS (Automatic Weather Station) exporter

This is a Prometheus exporter for fetching current data from the TMD (Thai Meteorological Department)'s observations.

## Usage

build and run the thing

```bash
cargo build --release

./target/release/tmd-aws-exporter
```

by default it queries data from AWS104 (Chaloem Phra Kiat Weather Observing Station), to change the AWS, set the envar

```bash
AWS_STATION_ID=37 ./target/release/tmd-aws-exporter
```
