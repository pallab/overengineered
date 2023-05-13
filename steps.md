``
cargo new webserver

cd webserver

npx create-next-app --js ui

✔ Would you like to use ESLint with this project? … No / Yes

✔ Would you like to use Tailwind CSS with this project? … No / Yes

✔ Would you like to use `src/` directory with this project? … No / Yes

✔ Use App Router (recommended)? … No / Yes

✔ Would you like to customize the default import alias? … No / Yes
``

change build step in package.json to 

```"build": "next build && next export -o ../static",```
This exports next files as static files which we can serve from actix

in next.config.js

``
const nextConfig = {
reactStrictMode: true,
images: {
unoptimized: true,
},
}
``

## Docker 

### build 
``
docker build -t overengineered . --progress=basic
``

### run
``
docker run -p 8080:8080  overengineered
``