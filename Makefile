hello_files := src/hello_world.c

hello-world hello-world.wasm: $(hello_files)
	emcc $^ -o $@

hello-world.wat: hello-world.wasm
	wasm2wat hello-world.wasm > hello-world.wat

run: hello-world
	node hello-world

clean:
	rm hello-world hello-world.wasm
