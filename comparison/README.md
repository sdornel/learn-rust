python3 -m venv venv
source venv/bin/activate


cargo run --release
python3 src/main.py
node src/main.js

Below are averages based on five runs of the respective code

FIRST TEST:
rust average - 819.0625248ms
python average - 0.4764327526s
node average - 0.7216s

SECOND TEST:
rust average - 2.1172037914s
python average - 13.908s
node average - 4.4744s

4.472 + 4.441 + 4.411 + 4.526 + 4.522