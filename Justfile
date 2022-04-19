alias t := test
alias c := check

cargo := `command -v cargo`

check:
    {{cargo}} clippy

test:
    {{cargo}} nextest run
