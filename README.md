<h1 align="center"> Next API Reference Generator </h1>

> NOTE: This crate is under development and in its very early stages. You may encounter bugs. Open up a issue or create a PR if you do.

This project aims to generate an API Reference of [Route Handler] found in `NextJS 13`.

Currently the aim is two provide atleast 2 generators:

- JSON
- Static HTML reference

The documentation will be updated as more work is done.

### Goals

- [ ] Basic JSON Generator
- [ ] Basic HTML Generator
- [x] Parse comments to add documentation to API endpoints\*.
- [x] Implement basic logging.

> Block comments are parsed but they still need more testing and work. Comments for named exports like `export { GET }` are unsupported at the moment.

### Contributing

If you find a feature you need in this project that is lacking or a bug, check if a issue is already created for it, if not go ahead and create a issue.

To contribute to the the project simply:

1. Fork the repository
2. Create a new branch
3. Do your magic
4. Submit the PR

A better workflow and guide will be created in the future if this project gains popularity.

[Route Handler]: https://nextjs.org/docs/app/building-your-application/routing/route-handlers
