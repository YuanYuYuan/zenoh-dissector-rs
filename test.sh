#!/usr/bin/env bash

set -e

rm ~/.local/lib/wireshark/plugins/4.0/epan/*.so

# ln -snf $(realpath ./target/debug/examples/libsimple_struct.so) ~/.local/lib/wireshark/plugins/4.0/epan/libsimple_struct.so
# cargo build --example simple_struct

# ln -snf $(realpath ./target/debug/examples/libdev_dissect.so) ~/.local/lib/wireshark/plugins/4.0/epan/libdev_dissect.so
# cargo build --example dev_dissect

ln -snf $(realpath ./target/debug/examples/libdev_zenoh.so) ~/.local/lib/wireshark/plugins/4.0/epan/libdev_zenoh.so
cargo build --example dev_zenoh

wireshark -r ./new-protocol.pcap
