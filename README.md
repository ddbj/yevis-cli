# yevis-cli

[![DOI](https://zenodo.org/badge/442338046.svg)](https://zenodo.org/badge/latestdoi/442338046)
[![Apache License](https://img.shields.io/badge/license-Apache%202.0-orange.svg?style=flat&color=important)](http://www.apache.org/licenses/LICENSE-2.0)

CLI tool to support building and maintaining Yevis workflow registry.

Features include:

- Generate a workflow metadata file template
- Validate the workflow metadata file
- Execute workflow tests
- Create a Pull Request to GitHub Repository
- Upload workflow-related files to [Zenodo](https://zenodo.org/) and obtain DOI
- Generate TRS responses ([GA4GH - Tool Registry Service API](https://www.ga4gh.org/news/tool-registry-service-api-enabling-an-interoperable-library-of-genomics-analysis-tools/)) and deploy them to GitHub Pages

In addition, see the below links:

- **[Publication on GigaScience](https://doi.org/10.1093/gigascience/giad006)**
- [`ddbj/workflow-registry`](https://github.com/ddbj/workflow-registry): a workflow registry built and maintained by [DDBJ](https://www.ddbj.nig.ac.jp/) using `yevis-cli`
- [`pitagora-network/yevis-DAT2-cwl`](https://github.com/pitagora-network/yevis-DAT2-cwl): a workflow registry for [DAT2-cwl](https://github.com/pitagora-network/DAT2-cwl) using `yevis-cli`
- [`sapporo-wes/yevis-web`](https://github.com/sapporo-wes/yevis-web): a web application to browse published workflows
- [`Yevis Getting Started`](https://sapporo-wes.github.io/yevis-cli/getting_started): the document for Yevis system installation and usage
- [`Yevis Getting Started Ja`](https://sapporo-wes.github.io/yevis-cli/getting_started_ja): 日本語での Yevis system の使い方

## Installation

**As a dependency, `yevis-cli` uses Docker to run tests.**

Use a single binary that is built without any dependencies (supports Linux only):

```bash
curl -fsSL -o ./yevis https://github.com/sapporo-wes/yevis-cli/releases/latest/download/yevis_$(uname -m)
chmod +x ./yevis
./yevis --help
```

Or, use the Docker environment:

```bash
curl -O https://raw.githubusercontent.com/sapporo-wes/yevis-cli/main/compose.yml
docker compose up -d
docker compose exec app yevis --help
```

## Usage

See [Getting Started - 3. Workflow Registration](https://sapporo-wes.github.io/yevis-cli/getting_started#3-workflow-registration) for a series of usages.

This section describes some subcommands.

```bash
$ yevis --help
yevis 0.5.8
DDBJ(Bioinformatics and DDBJ Center)
CLI tool that supports building a Yevis workflow registry with automated quality control

USAGE:
    yevis <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help             Prints this message or the help of the given subcommand(s)
    make-template    Generate a template file for the Yevis metadata file
    publish          Generate TRS responses and host them on GitHub Pages. (Basically used in the CI environment
                     (`CI=true`))
    pull-request     Create a pull request based on the Yevis metadata files (after validation and testing)
    test             Test workflow based on the Yevis metadata files
    upload-zenodo    Upload dataset to Zenodo
    validate         Validate schema and contents of the Yevis metadata file
```

### make-template

Generate a workflow metadata file template from a primary workflow file URL.

```bash
$ yevis make-template --help
yevis-make-template 0.5.8
Generate a template file for the Yevis metadata file

USAGE:
    yevis make-template [FLAGS] [OPTIONS] <workflow-location>

FLAGS:
    -h, --help              Prints help information
        --use-commit-url    Use `<commit_hash>` instead of `<branch_name>` in generated GitHub raw contents URLs
    -V, --version           Prints version information
    -v, --verbose           Verbose mode

OPTIONS:
        --gh-token <github-token>    GitHub Personal Access Token
    -o, --output <output>            Path to the output file [default: yevis-metadata.yml]

ARGS:
    <workflow-location>    Remote location of a primary workflow document
```

Workflow location is a URL like `https://github.com/sapporo-wes/yevis-cli/blob/main/tests/CWL/wf/trimming_and_qc.cwl`, which will later be converted to a raw URL like `https://raw.githubusercontent.com/sapporo-wes/yevis-cli/main/tests/CWL/wf/trimming_and_qc.cwl`.

`yevis-cli` collects various information and generates a template for the workflow metadata file.
In particular, `workflow.files` is generated as a recursive list of files from the primary workflow location.

### validate

Validate schema and contents of the workflow metadata file.

```bash
$ yevis validate --help
yevis-validate 0.5.8
Validate schema and contents of the Yevis metadata file

USAGE:
    yevis validate [FLAGS] [OPTIONS] [metadata-locations]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
        --gh-token <github-token>    GitHub Personal Access Token

ARGS:
    <metadata-locations>...    Location of the Yevis metadata files (local file path or remote URL) [default:
                               yevis-metadata.yml]
```

Explanation of validation rules for some fields:

| Field                       | Description                                                                                                                                                                    |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`                        | Workflow ID generated by `make-template` command. This value should not be changed.                                                                                            |
| `version`                   | Workflow version in the form of `x.y.z`.                                                                                                                                       |
| `license`                   | Workflow License. An example of a license should be a distributable license such as `CC0-1.0`, `MIT`, and `Apache-2.0`, because `yevis-cli` will later upload files to Zenodo. |
| `authors`                   | Workflow authors.                                                                                                                                                              |
| `authors.[].github_account` | GitHub account of the author.                                                                                                                                                  |
| `authors.[].name`           | Name of the author in the format `Family name, Given names` (e.g., `Doe, John`).                                                                                               |
| `authors.[].affiliation`    | Affiliation of the author (optional).                                                                                                                                          |
| `authors.[].orcid`          | ORCID of the author (optional).                                                                                                                                                |
| `workflow.name`             | Workflow name. Allowed characters are `a-z`, `A-Z`, `0-9`, `~!@#$%^&\*()\_+-={}[]\|:;,.<>?`, and space.                                                                        |
| `workflow.readme`           | Workflow readme.                                                                                                                                                               |
| `workflow.language`         | Choose from `CWL`, `WDL`, `NFL`, and `SMK`.                                                                                                                                    |
| `workflow.files`            | A list of files. At workflow runtime, files specified as `type: secondary` will be placed in the execution directory with `target` as a path.                                  |
| `workflow.testing`          | A list of tests. See `test` for how to write tests.                                                                                                                            |

Several examples are provided as follows:

- [`test-metadata-CWL.yml`](https://github.com/sapporo-wes/yevis-cli/blob/main/tests/test-metadata-CWL.yml)
- [`test-metadata-WDL.yml`](https://github.com/sapporo-wes/yevis-cli/blob/main/tests/test-metadata-WDL.yml)
- [`test-metadata-NFL.yml`](https://github.com/sapporo-wes/yevis-cli/blob/main/tests/test-metadata-NFL.yml)
- [`test-metadata-SMK.yml`](https://github.com/sapporo-wes/yevis-cli/blob/main/tests/test-metadata-SMK.yml)

### test

Test workflow using [GA4GH WES](https://www.ga4gh.org/news/ga4gh-wes-api-enables-portable-genomic-analysis/).

```bash
$ yevis test --help
yevis-test 0.5.8
Test workflow based on the Yevis metadata files

USAGE:
    yevis test [FLAGS] [OPTIONS] [metadata-locations]...

FLAGS:
    -f, --fetch-ro-crate    Fetch the execution results of the test run as RO-Crate. (Supported by Sapporo-
                            service>=1.4.0, generated at ./test-logs)
        --from-pr           Get modified files from a GitHub Pull Request. This option is used for pull request events
                            in the the CI environment. When using this option, specify a GitHub Pull Request URL (e.g.,
                            `${{ github.event.pull_request._links.html.href }}`) as `metadata_locations`
    -h, --help              Prints help information
    -V, --version           Prints version information
    -v, --verbose           Verbose mode

OPTIONS:
    -d, --docker-host <docker-host>      Location of the Docker host [default: unix:///var/run/docker.sock]
        --gh-token <github-token>        GitHub Personal Access Token
    -w, --wes-location <wes-location>    WES location where the test will be run. If not specified, `sapporo-service`
                                         will be started

ARGS:
    <metadata-locations>...    Location of the Yevis metadata files (local file path or remote URL) [default:
                               yevis-metadata.yml]
```

The tests are executed using WES.
If the option `--wes-location` is not specified, [`sapporo-service`](https://github.com/sapporo-wes/sapporo-service) will be started and used as WES.

An example of `workflow.testing` field is as follows:

```yaml
testing:
  - id: test_1
    files:
      - url: "https://example.com/path/to/wf_params.json"
        target: wf_params.json
        type: wf_params
      - url: "https://example.com/path/to/wf_engine_params.json"
        target: wf_engine_params.json
        type: wf_engine_params
      - url: "https://example.com/path/to/data.fq"
        target: data.fq
        type: other
```

There are three types of files:

| Type               | Description                                                 |
| ------------------ | ----------------------------------------------------------- |
| `wf_params`        | Workflow parameters file for the workflow execution.        |
| `wf_engine_params` | Workflow engine parameters file for the workflow execution. |
| `other`            | Other files. (e.g., data files, etc.)                       |

At WES runtime, the files specified as `wf_params` and `wf_engine_params` are placed as WES execution parameters.
In addition, the `other` files are placed in the execution directory with a `target` as a path.

The `id` field can be freely specified.

The `--from-pr` option is used within GitHub Actions.
See the GitHub Actions section.

The `--fetch-ro-crate` option is used to fetch the execution results of the test run as RO-Crate.
This option is supported by `sapporo-service>=1.4.0`.
The RO-Crate is generated at `./test-logs`.

### pull-request

Create a pull request after validation and testing.

```bash
$ yevis pull-request --help
yevis-pull-request 0.5.8
Create a pull request based on the Yevis metadata files (after validation and testing)

USAGE:
    yevis pull-request [FLAGS] [OPTIONS] --repository <repository> [metadata-locations]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -d, --docker-host <docker-host>      Location of the Docker host [default: unix:///var/run/docker.sock]
        --gh-token <github-token>        GitHub Personal Access Token
    -r, --repository <repository>        GitHub repository to which the pull request will be sent (format:
                                         <owner>/<repo>)
    -w, --wes-location <wes-location>    Location of a WES where the test will be run. If not specified, `sapporo-
                                         service` will be started

ARGS:
    <metadata-locations>...    Location of the Yevis metadata files (local file path or remote URL) [default:
                               yevis-metadata.yml]
```

A pull request is created from the forked repository as follows:

1. Fork a repository specified by the `--repository` option to your GitHub account
2. Create a new branch (named `workflow_id`) on the forked repository
3. Commit the workflow metadata file to the new branch
4. Create a pull request

### publish

Upload files to Zenodo, generate TRS responses and deploy them on GitHub Pages.

```bash
$ yevis publish --help
yevis-publish 0.5.8
Generate TRS responses and host them on GitHub Pages. (Basically used in the CI environment (`CI=true`))

USAGE:
    yevis publish [FLAGS] [OPTIONS] --repository <repository> [metadata-locations]...

FLAGS:
        --from-pr          Get modified files from GitHub Pull Request. This option is used for pull request events in
                           the CI environment. When using this option, specify GitHub Pull Request URL (e.g., `${{
                           github.event.pull_request._links.html.href }}`) as `metadata_locations`
    -h, --help             Prints help information
        --upload-zenodo    Upload dataset to Zenodo
    -V, --version          Prints version information
    -v, --verbose          Verbose mode
        --with-test        Test before publishing

OPTIONS:
    -d, --docker-host <docker-host>              Location of Docker host [default: unix:///var/run/docker.sock]
        --gh-token <github-token>                GitHub Personal Access Token
    -r, --repository <repository>                GitHub repository that publishes TRS responses (format: <owner>/<repo>)
    -w, --wes-location <wes-location>
            Location of the WES where the test will be run. If not specified, `sapporo-service` will be started

        --zenodo-community <zenodo-community>    Community set in Zenodo deposition

ARGS:
    <metadata-locations>...    Location of the Yevis metadata files (local file path or remote URL) [default:
                               yevis-metadata.yml]
```

This command is used within GitHub Actions.

Note that the following four options:

- `--from-pr`: Publish from a pull request ID
- `--upload-zenodo`: Upload workflow and dataset to Zenodo
- `--with-test`: Test before publishing

See the GitHub Actions section for more details.

### upload-zenodo

Upload files in the Yevis metadata to Zenodo and replace the metadata file with the Zenodo URL.

```bash
$ yevis upload-zenodo --help
yevis-upload-zenodo 0.5.8
Upload dataset to Zenodo

USAGE:
    yevis upload-zenodo [FLAGS] [OPTIONS] --repository <repository> [metadata-location]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
        --gh-token <github-token>                GitHub Personal Access Token
    -o, --output <output>                        Path to the output file [default: yevis-metadata-uploaded.yml]
    -r, --repository <repository>                GitHub repository that publishes TRS responses (format: <owner>/<repo>)
        --zenodo-community <zenodo-community>    Community set in Zenodo deposition
        --zenodo-host <zenodo-host>
            Zenodo host. Uses zenodo.org by default and sandbox.zenodo.org for dev-mode

        --zenodo-token <zenodo-token>
            Zenodo Personal Access Token. You can generate it at
            https://zenodo.org/account/settings/applications/tokens/new/

ARGS:
    <metadata-location>    Location of the Yevis metadata file (local file path or remote URL) [default: yevis-
                           metadata.yml]
```

#### Generated TRS Responses

Please note, as raised in the issue <https://github.com/ddbj/workflow-registry/issues/15> by @kinow, that the TRS responses generated by Yevis may not be fully compliant with the [TRS API](https://editor.swagger.io/?url=https://raw.githubusercontent.com/ga4gh/tool-registry-schemas/develop/openapi/openapi.yaml).

To summarize the comments in this issue:

The TRS API is designed to return the following:

```json
[
  {
    "path": "string",
    "file_type": "TEST_FILE",
    "checksum": {
      "checksum": "string",
      "type": "string"
    }
  }
]
```

from the `/tools/{id}/versions/{version_id}/{type}/files` endpoint. You can then use the `path` from this response to fetch the actual file by making a request to `/tools/{id}/versions/{version_id}/{type}/descriptor/{relative_path}`.

However, due to the features of Yevis, which include pre-generating the API responses and deploying them on GitHub Pages, it isn't capable of retrieving the actual file. Therefore, Yevis adjusts the `path` in the `/tools/{id}/versions/{version_id}/{type}/files` response to point directly to the actual file URL, as shown below:

```json
$ curl -fsSL https://ddbj.github.io/workflow-registry/tools/20da6ea5-de91-4973-ac20-216882357a0d/versions/1.0.0/NFL/files | jq .[0]
{
  "path": "https://zenodo.org/api/files/bb3e4bea-a1e5-49c8-8479-960299972c43/LICENSE",
  "file_type": "SECONDARY_DESCRIPTOR",
  "checksum": {
    "checksum": "5b70dc64d1f1bd721b6d62f587aa8d1f228d8ec128df58d32ae51cbac1347610",
    "type": "sha256"
  }
}
```

## GitHub Actions

`yevis-cli` uses GitHub Actions for CI/CD.

Two actions are provided as examples:

- [`yevis-test-pr.yml`](https://github.com/sapporo-wes/yevis-cli/blob/main/actions_example/yevis-test-pr.yml): Action to automatically validate and test a pull request
- [`yevis-publish-pr.yml`](https://github.com/sapporo-wes/yevis-cli/blob/main/actions_example/yevis-publish-pr.yml): Action to upload files to Zenodo and generate TRS responses when pull requests are merged
  - `ZENODO_TOKEN` must be set as GitHub Secrets.

Examples of `yevis-cli` commands executed within each action are as follows:

```bash
# yevis-test-pr.yml
$ yevis test \
    --verbose \
    --from-pr ${{github.event.pull_request._links.html.href }}

# yevis-publish-pr.yml
$ yevis publish \
    --verbose \
    --repository ${{ github.repository }} \
    --with-test \
    --from-pr ${{github.event.pull_request._links.html.href }} \
    --upload-zenodo
```

## Update workflow

Edit the Yevis metadata for existing workflows and follow the standard procedure using `yevis-cli`.
If the `workflow_id` in the metadata is the same, they are treated as the same workflow.
And the namespace is separated by the `version` as the TRS endpoint.

## Development

Launch a development environment using `docker compose`:

```bash
$ docker compose -f compose.dev.yml up -d --build
$ docker compose -f compose.dev.yml exec app bash
# cargo run -- --help
yevis 0.4.0
...
```

### Build binary

Recommendation, build the binary using `musl`:

```bash
$ docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:aarch64-musl cargo build --release

# No dependencies
$ ldd target/x86_64-unknown-linux-musl/release/yevis
statically linked
```

### Run test

Run unit tests:

```bash
cargo test -- --test-threads=1 --nocapture
```

Several test workflows are prepared.
See [tests/README.md](https://github.com/sapporo-wes/yevis-cli/blob/main/tests/README.md).

### Download artifacts from building GitHub Actions

```bash
gh run --repo sapporo-wes/yevis-cli list --workflow build_binary --json databaseId --jq .[0].databaseId | xargs -I {} gh run --repo sapporo-wes/yevis-cli download {} -n yevis
```

### Release

Use [`release.sh`](https://github.com/sapporo-wes/yevis-cli/blob/main/release.sh) as follows:

```bash
bash release.sh <new_version>
```

## License

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0).
See the [LICENSE](https://github.com/sapporo-wes/yevis-cli/blob/main/LICENSE).
