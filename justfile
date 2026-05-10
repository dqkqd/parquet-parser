[working-directory('books')]
books-test:
    - mdbook test

[working-directory('books')]
books-run:
    - mdbook serve --open

lint:
    - typos
    - cargo clippy --all-targets

style:
    - typos -w
    - cargo clippy --all-targets --fix --allow-dirty

test: books-test
    - cargo nextest run

test-all:
    - bash ./test-all-solutions.sh
