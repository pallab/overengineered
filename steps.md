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


## Diesel
brew install mysql-client 

which will give you the location of the lib 
then
RUSTFLAGS="-L/opt/homebrew/opt/mysql-client/lib" cargo install diesel_cli --no-default-features --features mysql

ref : https://stackoverflow.com/questions/54969208/how-to-link-mysql-client-installed-from-homebrew-with-diesel-cli

diesel setup --migration-dir database/migrations --database-url mysql://tenxdev:tenxpasswd@127.0.0.1:3306/overengineered

diesel migration generate create_users --migration-dir database/migrations

add sql to migration files 

diesel migration run --migration-dir database/migrations --database-url mysql://tenxdev:tenxpasswd@127.0.0.1:3306/overengineered

diesel print-schema --database-url mysql://tenxdev:tenxpasswd@127.0.0.1:3306/overengineered

### Protobuf
protoc --go_out=. --go-grpc_out=. proto/grpc.proto