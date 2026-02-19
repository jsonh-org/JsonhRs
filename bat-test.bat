:: Prevent quit on error
if not defined in_subprocess (cmd /k set in_subprocess=y ^& %0 %*) & exit
:: Clear screen
cls

:: Enable backtraces
set RUST_BACKTRACE=1

:: Test
cd jsonh_rs_tests
cargo test