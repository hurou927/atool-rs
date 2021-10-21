

test:
	cargo test

run:
	RATOOL_LOG=DEBUG argo run ls ./tests/resource/test_dir.tar.bz2

unpack:
	RATOOL_LOG=DEBUG argo run unpack ./tests/resource/test_dir.tar.bz2

pack:
	RATOOL_LOG=DEBUG cargo run pack ./tmp/test_unpack.tar.bz2 ./Makefile Cargo.toml


clean:
	rm -rf rapack*
