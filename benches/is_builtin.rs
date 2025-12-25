use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nodejs_built_in_modules::{BUILTINS, BUILTINS_WITH_MANDATORY_NODE_PREFIX};

/// Version using contains (current implementation)
fn is_nodejs_builtin_module_contains(specifier: &str) -> bool {
    if let Some(stripped) = specifier.strip_prefix("node:") {
        return BUILTINS.contains(&stripped)
            || BUILTINS_WITH_MANDATORY_NODE_PREFIX.contains(&stripped);
    }
    BUILTINS.contains(&specifier)
}

/// Version using binary_search (requires sorted arrays)
fn is_nodejs_builtin_module_binary_search(specifier: &str) -> bool {
    if let Some(stripped) = specifier.strip_prefix("node:") {
        return BUILTINS.binary_search(&stripped).is_ok()
            || BUILTINS_WITH_MANDATORY_NODE_PREFIX
                .binary_search(&stripped)
                .is_ok();
    }
    BUILTINS.binary_search(&specifier).is_ok()
}

static TEST_CASES: &[&str] = &[
    // All underscore-prefixed modules (beginning of sorted list)
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
    "node:_http_agent",
    "node:_stream_duplex",
    "node:_tls_wrap",
    // Common frequently-used modules
    "assert",
    "buffer",
    "child_process",
    "crypto",
    "events",
    "fs",
    "http",
    "https",
    "net",
    "os",
    "path",
    "process",
    "stream",
    "url",
    "util",
    "node:assert",
    "node:buffer",
    "node:crypto",
    "node:fs",
    "node:http",
    "node:path",
    "node:process",
    "node:stream",
    "node:util",
    // Modules with sub-paths
    "assert/strict",
    "dns/promises",
    "fs/promises",
    "path/posix",
    "path/win32",
    "readline/promises",
    "stream/consumers",
    "stream/promises",
    "stream/web",
    "timers/promises",
    "util/types",
    "node:assert/strict",
    "node:fs/promises",
    "node:stream/web",
    // Middle of the alphabet
    "inspector",
    "module",
    "punycode",
    "querystring",
    "readline",
    "repl",
    "node:inspector",
    "node:querystring",
    // End of sorted list
    "tty",
    "v8",
    "vm",
    "wasi",
    "worker_threads",
    "zlib",
    "node:vm",
    "node:worker_threads",
    "node:zlib",
    // Modules with mandatory node: prefix
    "node:sea",
    "node:sqlite",
    "node:test",
    "node:test/reporters",
    // Should fail - not built-ins
    "express",
    "lodash",
    "react",
    "typescript",
    "webpack",
    "node:express",
    "node:lodash",
    "node:react",
    // Should fail - typos/similar names
    "fss",
    "paths",
    "htp",
    "node:fss",
    "node:htp",
    // Should fail - beginning of alphabet
    "aaa",
    "abc",
    "node:aaa",
    // Should fail - middle of alphabet
    "mmm",
    "nnn",
    "node:mmm",
    // Should fail - end of alphabet
    "zzz",
    "zzzz",
    "node:zzz",
    // Should fail - underscore prefix but not built-in
    "_custom",
    "_my_module",
    "node:_custom",
    // Edge cases - empty-like
    "node:",
    // Common patterns in real code
    "node:dgram",
    "node:domain",
    "node:http2",
    "node:perf_hooks",
    "node:string_decoder",
    "node:trace_events",
    "async_hooks",
    "cluster",
    "console",
    "constants",
    "diagnostics_channel",
    "dgram",
    "dns",
    "domain",
    "http2",
    "inspector/promises",
    "perf_hooks",
    "string_decoder",
    "sys",
    "timers",
    "tls",
    "trace_events",
];

fn benchmark_is_builtin(c: &mut Criterion) {
    c.bench_function("is_builtin_contains", |b| {
        b.iter(|| {
            for case in TEST_CASES {
                black_box(is_nodejs_builtin_module_contains(black_box(case)));
            }
        });
    });

    c.bench_function("is_builtin_binary_search", |b| {
        b.iter(|| {
            for case in TEST_CASES {
                black_box(is_nodejs_builtin_module_binary_search(black_box(case)));
            }
        });
    });
}

criterion_group!(benches, benchmark_is_builtin);
criterion_main!(benches);
