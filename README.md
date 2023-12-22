# Website

My website :)

## Powered by

- [Axum](https://github.com/tokio-rs/axum)
- [Maud](https://maud.lambda.xyz/)

## Production

When running in production set the `production` feature flag.

### Environment variables

These must be set when running in production

- `PUBLIC_DIR`: Path to the `public` directory.

## Hosting

Hosted on my [website server](https://github.com/zkwinkle/website-server)

## Blog

To add more blog posts add the MD file corresponding to a post under [`src/components/blog_posts`](src/components/blog_posts).
Then, add it to the list in [`src/components/blog_posts.rs`](src/components/blog_posts.rs).
