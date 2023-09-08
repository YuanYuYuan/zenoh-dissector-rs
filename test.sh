#!/usr/bin/env bash

set -e
cargo build --example simple_struct
wireshark -r ./new-protocol.pcap
