
test:
	cargo test

run:
	cargo run ls ./tests/resource/test_dir.tar.bz2

unpack:
	cargo run unpack ./tests/resource/test_dir.tar.bz2
