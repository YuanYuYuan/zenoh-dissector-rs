# Zenoh Dissector in Rust!

**Experimental**

## Usage

Build the plugin

```bash
cargo build
```

Place the plugin properly

```bash
ln $(realpath ./target/debug/libwsdf_test.so) ~/.local/lib/wireshark/plugins/4.0/epan/libwsdf_test.so
```

## Sample Data

Reading a get/queryable sample

```bash
wireshark -r ./new-protocol.pcap
```

with the messages context decoded at './log.txt'.
