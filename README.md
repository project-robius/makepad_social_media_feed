# Makepad social media feed

This is an example application of mobile social media feed built with makepad

I'm currently running on android with the following commands:
First install [json-server](https://github.com/typicode/json-server) for the fake backend
```
npm install -g json-server
```
Run the server
```
json-server --watch db.json
```
Install cargo-makepad, in the root of the makepad repository run
```
cargo install --path ./tools/cargo_makepad
```
Run the application
```
cargo-makepad android run -p makepad_social_media_feed
```