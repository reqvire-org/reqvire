.PHONY: dev build preview clean

dev:
	npm run dev

build:
	npm run build

preview: build
	npm run preview -- --host 127.0.0.1 --port 4177

clean:
	rm -rf dist .vite-react-ssg-temp
