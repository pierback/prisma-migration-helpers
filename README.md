# prisma migration helpers

- binaries are pushed together with the source code thus no need to install rust toolchain
- but if one wants to build these binaries you need to have the rust toolchain installed.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- the build commands can be found in the `package.json`
- to install the dependencies run: `cargo install --path . --force`


## cleanup:
  - since Prisma [Prisma](https://www.prisma.io/docs/guides/database/advanced-database-tasks/sql-views-postgres) lacks proper support for sql views as of right now (25/04/2022). It always tries to add `create` and `alter` statements for views added as models in `schema.prisma` This script removes the create and alter statements in every new migration for all views found in the migration dir
 ```
npm run migration-cleanup
```

## commit:
  - before every commit this binary can be run to check if a new migration contains a create and alter statements for all views found in the migration dir
 ```
npm run commit
```

## version:
  - goes through migration dir and filters out the latest migration number 
  - based on the filtered out migration number returns next migration number 
 ```
npm run next-migration-version
```