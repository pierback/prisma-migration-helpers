# prisma migration helpers

- to build these binaries you need to have the rust toolchain installed.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- the build commands can be found in the `package.json`
- to install the dependencies run: `cargo install --path . --force`

## cleanup:
  - removes the create and alter statements in every new migration for all views found in the migration dir

## commit:
  - before every commit this binary is run to check if a new migration for all views found in the migration dir contains the create and alter statements mentioned above

## version:
  - goes through migration dir and filters out the latest migration number 
  - based on the filtered out migration number returns next migration number 