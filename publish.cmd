@echo off
setlocal enableDelayedExpansion

echo current path: !cd!
cd /d !cd!

call cargo publish --manifest-path Cargo.toml

endlocal
