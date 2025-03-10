ifneq ("$(wildcard .env)","")
	include .env
	export $(shell grep -v '^#' .env | xargs)
endif

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory
libpath = ./$(shell find target -name "libapp*.so"|grep -m1 debug|xargs dirname)

all: check run-std test run test-c

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
	gcc -o use_shared -O3 \
		-Wl,-rpath='$$ORIGIN',-rpath='$$ORIGIN/lib',-rpath='$$ORIGIN/../lib',-rpath='$(libpath)' \
		-o target/test_lib_c tests/test_lib.c \
		-L$(libpath) -lapp_nostd
	./target/test_lib_c

show-symbols:
	nm -g $(args) \
		$(libpath)/app-nostd \
		$(libpath)/libapp_nostd.so

show-symbols-dyn:
	$(make) show-symbols args="-D $(args)"

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
