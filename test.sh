#!/usr/bin/env bash

set -e

rm -f ~/.local/lib/wireshark/plugins/zenoh.lua
rm -f ~/.local/lib/wireshark/plugins/4.0/epan/*.so

# cargo build --example simple_struct
# ln -snf $(realpath ./target/debug/examples/libsimple_struct.so) ~/.local/lib/wireshark/plugins/4.0/epan/libsimple_struct.so

# cargo build --example dev_dissect
# ln -snf $(realpath ./target/debug/examples/libdev_dissect.so) ~/.local/lib/wireshark/plugins/4.0/epan/libdev_dissect.so

cargo build --example dev_zenoh
ln -snf $(realpath ./target/debug/examples/libdev_zenoh.so) ~/.local/lib/wireshark/plugins/4.0/epan/libdev_zenoh.so

# cargo build --example dev_zenoh --release
# ln -snf $(realpath ./target/release/examples/libdev_zenoh.so) ~/.local/lib/wireshark/plugins/4.0/epan/libdev_zenoh.so

wireshark -r ./new-protocol.pcap
# wireshark -r ./new-protocol-sub.pcap
# wireshark -r ./new-protocol-sub-512KiB.pcap
# wireshark -r /home/circle/Workings/ZettaScale/src/zenoh/new-protocol-sub-16KiB.pcap
