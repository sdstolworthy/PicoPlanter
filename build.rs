[package]
edition = "2021"
name = "rp2040-project-template"
version = "0.1.0"
license = "MIT OR Apache-2.0"

[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
embedded-hal = { version = "0.2.5", features = ["unproven"] }

defmt = "0.3"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }

# We're using a Pico by default on this template
rp-pico = "0.8"
rp2040-hal = "0.7.0"
dht-sensor = "0.2.1"

# but you can use any BSP. Uncomment this to use the pro_micro_rp2040 BSP instead
# sparkfun-pro-micro-rp2040 = "0.7"

# If you're not going to use a Board Support Package you'll need these:
# rp2040-hal = { version="0.9", features=["rt", "critical-section-impl"] }
# rp2040-boot2 = "0.3"

# cargo build/run
[profile.dev]
codegen-units = 1
debug = 2
debug-assertions = true
incremental = false
opt-level = 3
overflow-checks = true

# cargo build/run --release
[profile.release]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 3
overflow-checks = false

# do not optimize proc-macro crates = faster builds from scratch
[profile.dev.build-override]
codegen-units = 8
debug = false
debug-assertions = false
opt-level = 0
overflow-checks = false

[profile.release.build-override]
codegen-units = 8
debug = false
debug-assertions = false
opt-level = 0
overflow-checks = false

# cargo test
[profile.test]
codegen-units = 1
debug = 2
debug-assertions = true
incremental = false
opt-level = 3
overflow-checks = true

# cargo test --release
[profile.bench]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 3
