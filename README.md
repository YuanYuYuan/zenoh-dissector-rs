# Zenoh Dissector in Rust!

**Experimental**

## Usage

Build the plugin

```bash
cargo build
```

Place the plugin properly

```bash
ln -snf $(realpath ./target/debug/libzenoh_dissector.so) ~/.local/lib/wireshark/plugins/4.0/epan/libzenoh_dissector.so
```

## Sample Data

Reading a get/queryable sample

```bash
wireshark -r ./new-protocol.pcap
```

with the messages context decoded at './log.txt'.
