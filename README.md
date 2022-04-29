# Discord API Endpoints Extractor

A little rust tool that walks over all the files in the downloaded zip
of `discord/discord-api-docs@main`, and extracts documented endpoints from
the markdown files.

All endpoints are printed to stdout in the following format

```yaml
/name/of/endpoint:
  - METHOD-1
  - METHOD-2
```

## How to use?

## You need

- Rust
- Cargo

### Extracting endpoints

Firstly, execute `cargo build`\
This installs all dependencies and creates the necessary executeable
at `./target/debug/discord-api-endpoints-extractor{.extension}`

Now that you have the executeable, you can simply run\
`./target/debug/{executeable} > your_yaml_file.yaml`
