.PHONY: wasm

wasm:
	wasm-pack build --target web -d ./vite-project/src/pkg

build:
	cd ./vite-project && npm run build

dev:
	cd ./vite-project && npm run dev