
server-update: build
	sudo systemctl restart mwpf-web

build:
	cargo build --release
	cd frontend && npm install && npm run build
