-include .env
export

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory
libpath = ./$(shell find target -name "libapp*.so" -path "*/debug/*" -exec dirname "{}" \; 2>/dev/null | head -n1)

all: clean run-std test run test-c check

run:
	cargo run $(args)

run-std:
	$(make) run args="--no-default-features $(args)"

check:
	cargo clippy

clean:
	rm -fr target/*

test:
	RUSTFLAGS="-Zpanic_abort_tests -Cpanic=abort" \
	cargo +nightly test --no-default-features $(args) -- --nocapture --color always

test-c:
	gcc -o use_shared -Oz \
		-Wl,-z,relro,-z,now,-rpath='$$ORIGIN',-rpath='$$ORIGIN/lib',-rpath='$$ORIGIN/../lib',-rpath='$(libpath)' \
		-L$(libpath) -lapp_nostd \
		-o target/test_lib_c tests/test_lib.c
	./target/test_lib_c

symbols:
	nm -g $(args) \
		$(libpath)/app-nostd \
		$(libpath)/libapp_nostd.so

symbols-dyn:
	$(make) symbols args="-D $(args)"

env:
	env

config:
	@echo libpath = $(libpath)

help:
	@echo -e "\
	Usage guide:\n\n\
	make all		- Build, run, testing\n\
	make run		- Run without 'std'\n\
	make run-std		- Run with 'std'\n\
	make check		- Check code\n\
	make clean		- Clean target directory\n\
	make test		- Test Rust code\n\
	make test-c		- Test C code\n\
	make show-symbols	- Show library symbols\n\
	make show-symbols-dyn	- Show only dynamic library symbols\
	"
