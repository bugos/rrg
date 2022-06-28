// Copyright 2020 Google LLC
//
// Use of this source code is governed by an MIT-style license that can be found
// in the LICENSE file or at https://opensource.org/licenses/MIT.

use std::path::PathBuf;

const PROTOS: &'static [&'static str] = &[
    "vendor/grr/grr/proto/grr_response_proto/semantic.proto",
    "vendor/grr/grr/proto/grr_response_proto/sysinfo.proto",
    "vendor/grr/grr/proto/grr_response_proto/knowledge_base.proto",
    "vendor/grr/grr/proto/grr_response_proto/jobs.proto",
    "vendor/grr/grr/proto/grr_response_proto/timeline.proto",
    "vendor/grr/grr/proto/grr_response_proto/anomaly.proto",
    "vendor/grr/grr/proto/grr_response_proto/export.proto",
    "vendor/grr/grr/proto/grr_response_proto/objects.proto",
    "vendor/grr/grr/proto/grr_response_proto/output_plugin.proto",
    "vendor/grr/grr/proto/grr_response_proto/flows.proto",
    "vendor/grr/grr/proto/grr_response_proto/sysinfo.proto",
    "vendor/grr/grr/proto/grr_response_proto/user.proto",
];

const INCLUDES: &'static [&'static str] = &[
    "vendor/grr/grr/proto",
];

fn main() {
    // `prost` generated code.
    prost_build::compile_protos(&PROTOS, &INCLUDES)
        .expect("failed to compile proto files");

    // There is a problem with one enum generated by PROST!: it's values use
    // name mangling, but it's default value does not. This is likely a bug in
    // PROST! itself, but for now we hack around it by replacing the spurious
    // line in the output file ourselves.
    let outdir: PathBuf = std::env::var("OUT_DIR")
        .expect("no output directory")
        .into();

    let target = outdir.join("grr.rs");

    let grr = std::fs::read_to_string(&target)
        .expect("failed to read the generated output file")
        .replace("TskFsAttrTypeDefault", "Default");

    std::fs::write(&target, grr)
        .expect("failed to write updated output file");

    // `protobuf` generated code.
    let proto_out_dir = outdir.join("proto");
    std::fs::create_dir_all(&proto_out_dir).unwrap();

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: &proto_out_dir.to_string_lossy(),
        includes: &[
            "vendor/grr/grr/proto",
            "vendor/protobuf/src"
        ],
        input: PROTOS,
        ..Default::default()
    }).unwrap();

    std::fs::write(proto_out_dir.join("mod.rs"), b"
        pub mod jobs;
        pub mod sysinfo;
        pub mod timeline;
        pub mod user;
        pub mod knowledge_base;
    ").unwrap();
}
