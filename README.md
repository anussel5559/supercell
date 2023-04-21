# Supercell
## tl;dr
Supercell is a CLI software tool for load testing APIs that can generate heavy traffic and stress to identify performance issues. It's highly configurable and scalable, and provides detailed reports and analytics for optimization. The name comes from the weather term for a thunderstorm, reflecting its ability to simulate heavy activity quickly and potentially disrupt it's target in unexpected ways.

## Installation
Currently, the only binary release target for `supercell` is Apple Silicon. Head to the github releases page to grab the latest binary. You'll need to drop it in an accessible folder such as `~/bin` and ensure that directory is in your terminal's `PATH` like:
```
export PATH=~/bin:$PATH
```
at which point running `supercell` should result in a help text!

## Configuration
Currently supercell accepts one command argument: a filepath to a JSON file which will specify the parameters for the test run.

The schema for this file is fairly simple:
```json
{
  "request": {
    "method": "enum, one of GET, POST, PATCH, or DELETE",
    "host": "string, like https://google.com",
    "path": "string, a request path",
    "header": {
      "authorization": "string, the authorization header to use. This is required, so leave empty if no authorization header"
    }
  },
  "test_parameters": {
    "threads": "number, specifies the number of concurrent worker threads to make requests",
    "requests": "number, the TOTAL number of requests to make",
    "timeout_ms": "number, the timeout to adhere to for EACH request made"
  }
}
```

Once your test file is setup, simple run the CLI:
```sh
supercell /path/to/test.json
```

## Future
- [ ] Write raw results to a file
- [ ] Summary by status code
