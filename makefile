PHONY: jwtkey, svr

jwtkey:
	sh script/gen_jwtkey.sh

svr:
	cargo build --package pf --package game --package gate
