#!/usr/bin/env bash

cargo build
wireshark -r /home/circle/Workings/ZettaScale/project/dissector/new-protocol.pcap
