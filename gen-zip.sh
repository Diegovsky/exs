#!/usr/bin/env fish
cargo -Z unstable-options build --release --artifact-dir=bin
zip -r tarefa.zip (ls . | rg -v 'target|gen-zip.sh')
