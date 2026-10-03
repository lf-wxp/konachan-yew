// Runtime configuration for the Konachan web bundle.
//
// This file is loaded by index.html before the WASM bundle boots. The keys are
// optional: when a key is missing the app falls back to its compile-time
// default (e.g. the `safe` cargo feature).
//
// The Docker image regenerates this file from environment variables on every
// container start, so the same image can run in regular or safe mode:
//
//   docker run -e KONACHAN_SAFE=true ...
window.__KONACHAN_CONFIG__ = {};
