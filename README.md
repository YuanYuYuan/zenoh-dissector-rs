# Zenoh Dissector in Rust!

**Experimental**

## Usage

Build the plugin

```bash
cargo build
```

Place the plugin properly

```bash
ln $(realpath ./target/debug/libzenoh_dissector_rs.so) ~/.local/lib/wireshark/plugins/4.0/epan/libzenoh_dissector_rs.so
```

## Sample Data

Reading a get/queryable sample

```bash
wireshark -r ./new-protocol.pcap
```

with the messages context decoded at './log.txt'.
