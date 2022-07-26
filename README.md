# sidemount
🏕 sidemount 🤿 🏝⏛ is a streamlined http/web toolkit designed for building async network apps in Rust 🦀

## Examples

### Routing

To get started with routing in sidemount you will make use of the `sidemount::router`. This primarily allows you to setup a [radix tree](https://en.wikipedia.org/wiki/Radix_tree) backed routing implementation. Each node that has an associated handler will let you attach different **Methods** to it. So long as the actual handler `impl Handler` it will work. You can also chain handlers with the `tuple` syntax such as `(handler, handler2)`. This allows you to create middleware without having to worry about additional overhead.

Finally, if you wish to mount additional functions prior or after a particular path is executed use the `.mount` method on the router. This method will also accept any `impl Handler` and supports the `tuple` syntax here as well. You can call the `.mount` method as many times as you like (but it is generally preferred to use the tuple syntax for lower overhead when possible.

```rust
let mut router = sidemount::router();
router.mount((tester, tester2));
router.mount(tester2);
router.at("/foo/bar").get(tester);
router.at("/foo/bar/baz").get((tester, tester2));

let mut sub_router = sidemount::router();
sub_router.at("/bleh").get(tester);
sub_router.at("/foo/bar").post(tester);
router.route("/hi", sub_router);

assert!(router.find("/hi/bleh", Method::GET).is_found());
assert!(router.find("/hi/foo/bar", Method::POST).is_found());
assert!(router.find("/foo/bar", Method::GET).is_found());
assert!(router.find("/foo/bar/baz", Method::GET).is_found());
```

