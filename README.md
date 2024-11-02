# Personal Website

My website :)

## Powered by

- [Axum](https://github.com/tokio-rs/axum)
- [Maud](https://maud.lambda.xyz/)

## Running locally

To watch:
```
cargo w
```

Or just build once and run:
```
cargo run
```

Then go to http://localhost:31415.

## Production

When running in production set the `production` feature flag.

Website can be found at https://zkwinkle.is-a.dev.

### Environment variables

These must be set when running in production

- `PUBLIC_DIR`: Path to the `public` directory.

## Hosting

Hosted on my [personal server server](https://github.com/zkwinkle/personal-server)

## Blog

To add more blog posts add the MD file corresponding to a post under [`src/components/blog_posts`](src/components/blog_posts).
Then, add it to the list in [`src/components/blog_posts.rs`](src/components/blog_posts.rs).

## Ideas

Just compiling random ideas I get for the website :)

- Photography page
- Lists page (restaurants, movies, idk)
- CV

### Blog

- Add auto-generated table of contents and anchors
- Why Rust good
