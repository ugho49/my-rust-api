#!/usr/bin/env bash

# For an automatic reloading experience you need to install cargo-watch and systemfd
# --> cargo install systemfd cargo-watch
systemfd --no-pid -s http::3000 -- cargo watch -x run
