use std::borrow::Cow;

use nodejs_built_in_modules::{
    BUILTINS, BUILTINS_WITH_MANDATORY_NODE_PREFIX, is_nodejs_builtin_module,
};

#[test]
fn test_alphabetization() {
    assert!(BUILTINS.is_sorted());
    assert!(BUILTINS_WITH_MANDATORY_NODE_PREFIX.is_sorted());
}

#[test]
fn pass() {
    let pass = [
        "_http_agent",
        "_http_client",
        "_http_common",
        "_http_incoming",
        "_http_outgoing",
        "_http_server",
        "_stream_duplex",
        "_stream_passthrough",
        "_stream_readable",
        "_stream_transform",
        "_stream_wrap",
        "_stream_writable",
        "_tls_common",
        "_tls_wrap",
        "assert",
        "assert/strict",
        "async_hooks",
        "buffer",
        "child_process",
        "cluster",
        "console",
        "constants",
        "crypto",
        "dgram",
        "diagnostics_channel",
        "dns",
        "dns/promises",
        "domain",
        "events",
        "fs",
        "fs/promises",
        "http",
        "http2",
        "https",
        "inspector",
        "inspector/promises",
        "module",
        "net",
        "os",
        "path",
        "path/posix",
        "path/win32",
        "perf_hooks",
        "process",
        "punycode",
        "querystring",
        "readline",
        "readline/promises",
        "repl",
        "stream",
        "stream/consumers",
        "stream/promises",
        "stream/web",
        "string_decoder",
        "sys",
        "timers",
        "timers/promises",
        "tls",
        "trace_events",
        "tty",
        "url",
        "util",
        "util/types",
        "v8",
        "vm",
        "wasi",
        "worker_threads",
        "zlib",
    ];

    for specifier in pass {
        for specifier in [
            Cow::Borrowed(specifier),
            Cow::Owned(format!("node:{specifier}")),
        ] {
            assert!(is_nodejs_builtin_module(&specifier));
        }
    }

    let prefixed = [
        "node:sea",
        "node:sqlite",
        "node:test",
        "node:test/reporters",
    ];

    for specifier in prefixed {
        assert!(is_nodejs_builtin_module(specifier));
    }
}

#[test]
fn fail() {
    let fail = [
        "node",
        "node:",
        "node:aaa",
        "aaa",
        "sea",
        "sqlite",
        "test",
        "test/reporters",
    ];

    for specifier in fail {
        assert!(!is_nodejs_builtin_module(specifier));
    }
}
