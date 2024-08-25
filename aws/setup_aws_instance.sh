#!/bin/bash

set -ex

# install dependencies
sudo yum install -y gcc gnuplot
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# init the benchmark setup
git submodule update --init --recursive


