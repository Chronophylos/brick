alias t := test
alias c := check

cargo := `command -v cargo`

check:
    {{cargo}} +nightly clippy

test:
    {{cargo}} nextest run