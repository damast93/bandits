.PHONY: all clean rust

all: rust plots/bandits.png

rust: bin/bandits_rust

bin/bandits: bin/bandits_rust
	ln bin/bandits_rust bin/bandits -f

bin/bandits_rust: src/rust/target/release/rust
	\cp src/rust/target/release/rust bin/bandits_rust

src/rust/target/release/rust:
	cd src/rust/ &&	cargo build --release

bandits.csv: bin/bandits
	bin/bandits > bandits.csv 

plots/bandits.png: bandits.csv
	python plot.py bandits.csv plots/bandits.png

clean:
	rm -f bin/*
	rm -f plots/*
	rm -fr src/rust/target
