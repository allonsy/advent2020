#!/bin/bash

DAY=$1

mkdir day$DAY

cd day$DAY
cargo init --bin
touch input.txt
cp src/main.rs src/main1.rs
mv src/main.rs src/main2.rs

cat << END > Cargo.toml
[package]
name = "day$DAY"
version = "0.1.0"
authors = ["Alec Snyder <linuxbash8@gmail.com>"]
edition = "2018"

[dependencies]
util = { path = "../util" }

[[bin]]
name = "main1"
path = "src/main1.rs"

[[bin]]
name = "main2"
path = "src/main2.rs"
END