.PHONY: build server-update

build:
	cargo build --release
	cd frontend && npm install && npm run build

server-update: build
	sudo systemctl restart mwpf-web
