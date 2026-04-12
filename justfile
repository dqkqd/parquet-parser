lint:
    - cargo clippy --all-targets

fix:
    - cargo clippy --all-targets --fix --allow-dirty
