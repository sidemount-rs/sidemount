# sidemount
🏕 sidemount 🤿 🏝⏛ is a streamlined http/web toolkit designed for building async network apps in Rust 🦀

```rust
#[get("/{username}")]
fn posts(req: Request) -> Response {
    Response::default()
}

#[get("/")]
fn index(req: Request) -> Response {
    Response::default()
}

#[get("/settings")]
fn settings(req: Request) -> Response {
    Response::default()
}

fn explore(req: Request) -> Response {
    Response::default()
}

#[tokio::main]
async fn main() {
    let mut app = sidemount::new();
    app.route("/", routes![index, settings, posts]);
    app.at("/explore").get(explore);
    app.listen("127.0.0.1:7000").await
}
```

### Routing

To get started with routing in sidemount you will make use of the `sidemount::router`. This primarily allows you to setup a [radix tree](https://en.wikipedia.org/wiki/Radix_tree) backed routing implementation. Each node that has an associated handler will let you attach different **Methods** to it. So long as the actual handler `impl Handler` it will work. You can also chain handlers with the `tuple` syntax such as `(handler, handler2)`. This allows you to create middleware without having to worry about additional overhead.

> at("/path/to").(get | post | ...)

```rust
fn tester() {}
fn tester2() {}

let mut router = sidemount::router();
router.at("/foo/bar").get(tester);
router.at("/foo/bar/baz").get((tester, tester2));
```

> .mount(impl Handler)

If you wish to mount additional functions prior or after a particular path is executed use the `.mount` method on the router. This method will also accept any `impl Handler` and supports the `tuple` syntax here as well. You can call the `.mount` method as many times as you like (but it is generally preferred to use the tuple syntax for lower overhead when possible.

```rust
fn tester() {}
fn tester2() {}

let mut router = sidemount::router();
router.mount((tester, tester2));
router.mount(tester2);
router.at("/foo/bar").get(tester);
router.at("/foo/bar/baz").get((tester, tester2));
```

> .route("/path", router)

A router can also be routed to additional sub-routers using the `.route` method. Simple create a new router using `sidemount::router()` and pass it along with the path on `.route`.

```rust
let mut sub_router = sidemount::router();
sub_router.at("/bleh").get(tester);
sub_router.at("/foo/bar").post(tester);
router.route("/hi", sub_router);

```

> .find("/path/to/route", METHOD) -> RouteResult

The router has a method it for finding existing routes using the `.find` method, passing along the path and the `Method` to look for an associated `RouteResult`.

```rust
assert!(router.find("/hi/bleh", Method::GET).is_found());
assert!(router.find("/hi/foo/bar", Method::POST).is_found());
assert!(router.find("/foo/bar", Method::GET).is_found());
assert!(router.find("/foo/bar/baz", Method::GET).is_found());
```

