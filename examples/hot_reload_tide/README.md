# A hot reloading web server

Imagine this is a web app that remembers information about audio messages.
It has a [configuration](./config.json) file that acts as a database, you can edit the configuration and the app will pick up changes without the need to restart it.

You can use this concept to make your hot-reloading web servers. It uses a mutex under an atomic reference counted pointer to guarantee that the config won't be read and written to at the same time. To learn about how that works, please check out the [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) chapter of the Rust book.

To try the app, please run

```sh
cargo run
```

open [this link](http://localhost:8080/messages) in your browser,
then try editing the [config.json](./config.json) file and reload the browser.
You should see the response in your browser reflect the latest changes without restarting the app.
