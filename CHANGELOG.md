# Changelog

## Rust v3.0.0
- 🔥 **BREAKING** If you didn't create `GetContentRequest` by spreading default (as in the content example) you must specify the new field `include_unpublished_intents` as well now.
- 🔥 **BREAKING** The field `GetContentRequest.threshold` is explcitly marked as optional now so it has to be set to `Some(f32)` (or `None`) instead of `f32` now.
- Added `ContentResponseItem.matched_inputs` which counts the number of inputs of the intent that matched.
- Added `ContentResponseItem.matched_keyword` which returns the keywords that resulted in the match.
- The content example allows to provide a different prompt as first command line argument now.
- Bumped dependencies.
## NodeJS v2.1.0
- Added `GetContentRequest.include_unpublished_intents` to include unpublished intents in the response.
- The field `GetContentRequest.threshold` is explcitly marked as optional now so if not specified, the server will default to the project's default threshold.
- Added `ContentResponseItem.matched_inputs` which counts the number of inputs of the intent that matched.
- Added `ContentResponseItem.matched_keyword` which returns the keywords that resulted in the match.
- Bumped dependencies.
## Python v2.1.0
- Added `GetContentRequest.include_unpublished_intents` to include unpublished intents in the response.
- The field `GetContentRequest.threshold` is explcitly marked as optional now so if not specified, the server will default to the project's default threshold.
- Added `ContentResponseItem.matched_inputs` which counts the number of inputs of the intent that matched.
- Added `ContentResponseItem.matched_keyword` which returns the keywords that resulted in the match.

## Node v2.0.1 - 2025-08-25
- Bumped dependencies.
## Rust v2.0.1 - 2025-08-25
- Bumped dependencies.

## Rust v2.0.0 - 2025-07-25
- Updated to latest protobuf API.

## NodeJS v2.0.0 - 2025-07-25
- Updated dependencies.
- Updated to latest protobuf API.

## Python v2.0.0 - 2025-07-25
- Updated to latest protobuf API.
- Added close method to the client to allow graceful shutdown of the connection.

## NodeJS v1.1.1 - 2025-05-16
- Bumped dependencies.
## Rust v1.0.2 - 2025-05-16
- Bumped dependencies.
- Fixed connection issues with non TLS endpoints when specifying empty token or secret.

## Python v1.0.1 - 2025-04-29
- Updated documentation.

## Python v1.0.0 - 2025-04-29
- Initial release of the Aristech TTS-Client for Python.

## NodeJS v1.1.0 - 2025-04-29
- Updated dependencies.
- Deprecated `process` in favor of `runFunctions` for better cohesion with the protobuf API.

## Rust v1.0.1 - 2025-04-29
- Updated dependencies.
- Removed unused dependencies.


## Rust v1.0.0 - 2024-10-29
- Initial release of the Aristech TTS-Client for Rust.

## NodeJS v1.0.0 - 2024-10-29
- Initial release of the Aristech TTS-Client for NodeJS.