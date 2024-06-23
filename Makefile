hello_files := src/hello_world.c

hello-world: $(hello_files)
	emcc $^ -o $@

run: hello-world
	node hello-world

clean:
	rm hello-world hello-world.wasm
