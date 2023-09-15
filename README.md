# Zenoh Dissector in Rust!

**Experimental**

## Usage


Build the plugin

```bash
cargo build --release
```

Place the plugin properly

(Linux)
```bash
mkdir -p ~/.local/lib/wireshark/plugins/4.0/epan
cp ./target/release/libzenoh_dissector.so ~/.local/lib/wireshark/plugins/4.0/epan/libzenoh_dissector.so
```

(MacOS)
Assuming users have Wireshark application installed under _/Applications/Wireshark.app_.

```bash
cp ./target/release/libzenoh_dissector.dylib /Applications/Wireshark.app/Contents/PlugIns/wireshark/4-0/epan/libzenoh_dissector.so
```


## Sample Data

Reading a get/queryable sample

```bash
wireshark -r ./new-protocol.pcap
```

with the messages context decoded at './log.txt'.
