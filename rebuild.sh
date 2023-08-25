#!/usr/bin/env bash

cargo build && wireshark -r ./new-protocol.pcap
