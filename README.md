<h1 align="center"> Next API Reference Generator </h1>

> NOTE: This crate is under development and in its very early stages. You may encounter bugs. Open up a issue or create a PR if you do.

This project aims to generate an API Reference of [Route Handler] found in `NextJS 13`.

Currently the aim is to provide atleast 2 generators:

- JSON
- Static HTML reference

The documentation will be updated as more work is done.

```sh
Usage: next_api_reference [OPTIONS] --output <OUTPUT>

Options:
  -v, --verbose              Enable verbose logging
  -l, --location <LOCATION>  Location to find route handlers from [default: ./]
  -o, --output <OUTPUT>      The output location
  -h, --help                 Print help
  -V, --version              Print version
```

### Goals

- [x] Basic JSON Generator.
- [x] Basic HTML Generator.
- [x] Parse comments to add documentation to API endpoints.
- [x] Implement basic logging.
- [ ] Better comment parsing.

### JSON Generator

The JSON generator outputs your api reference to a JSON file. It is meant to be used in case you want to write your own api reference site. You can use the data generated from this generator and parse it to create a reference site that suites your need.

#### Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Schema for next_api_reference json generator",
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string"
      },
      "method_metadata": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "method_type": {
              "type": "string"
            },
            "comment": {
              "type": "array",
              "items": {
                "type": "string"
              }
            }
          },
          "required": ["method_type"]
        }
      }
    },
    "required": ["path", "method_metadata"]
  }
}
```

#### Sample output

```json
[
  {
    "path": "/api/items",
    "method_metadata": [
      {
        "method_type": "GET",
        "comment": ["Get a list of all items"]
      },
      {
        "method_type": "POST",
        "comment": ["Create an item"]
      },
      {
        "method_type": "DELETE",
        "comment": ["Delete an item or a list of items"]
      }
    ]
  },
  {
    "path": "/api/items/count",
    "method_metadata": [
      {
        "method_type": "GET",
        "comment": ["Get a count of all items"]
      }
    ]
  }
]
```

### Docstring

Docstring parsing is supported by this crate. However it is very crude at the moment.

#### Supported patterns

To create a docstring simply create a single line comment **above** the definition of the functon:

```ts
// This is a simple docstring
export async function GET(request: Request) {}
```

Block comments, while supported work differently in the different generators. For the HTML generator, only the first line will be shown, as for the JSON generator the entire string is provided in raw form and **may** require some extra processing.

> The behavior of block comments in the HTML generator **might** change in the future.

#### Unsupported patterns

Unfortunately docstring parsing for something like this is currently not supported and the docstring will simply be ignored:

```ts
// This is a simple docstring
async function GET(request: Request) {}

export { GET };
```

Block commen

### Contributing

If you find a feature you need in this project that is lacking or a bug, check if a issue is already created for it, if not go ahead and create a issue.

To contribute to the the project simply:

1. Fork the repository
2. Create a new branch
3. Do your magic
4. Submit the PR

A better workflow and guide will be created in the future if this project gains popularity.

[Route Handler]: https://nextjs.org/docs/app/building-your-application/routing/route-handlers
