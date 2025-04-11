python3 -m venv venv
source venv/bin/activate


cargo run --release
python3 src/main.py
node src/main.js

Below are averages based on five runs of the respective code

FIRST TEST (scraping only):
rust average - 819.0625248ms
python average - 0.4764327526s
node average - 0.7216s

SECOND TEST (scrapes and gets word count):
rust average - 2.1172037914s
python average - 13.908s
node average - 4.4744s

THIRD TEST (print longest 3 summaries per page):
rust average - 2.1418013914s
python average - 13.504s
node average - 4.427s
