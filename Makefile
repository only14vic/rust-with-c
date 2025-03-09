-include .env

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory
libpath = ./$(shell find target -name "libapp*.so"|grep -m1 debug|xargs dirname)

run:
	cargo run $(args)

run-std:
	$(make) run args=--no-default-features

check:
	cargo clippy

clean:
	rm -fr target/*

test:
	RUSTFLAGS="-Zpanic_abort_tests -Cpanic=abort" \
	cargo +nightly test --no-default-features $(args) -- --nocapture --color always

test-c:
	gcc -o use_shared -O3 \
		-Wl,-rpath='$$ORIGIN',-rpath='$$ORIGIN/lib',-rpath='$$ORIGIN/../lib',-rpath='$(libpath)' \
		-o target/test_lib_c tests/test_lib.c \
		-L$(libpath) -lapp_nostd
	./target/test_lib_c

show-symbols:
	nm -g $(args) \
		$(libpath)/app-nostd \
		$(libpath)/libapp_nostd.so

show-symbols-dynamic:
	$(make) show-symbols args=-D

help:
	@echo "Usage guide:"
