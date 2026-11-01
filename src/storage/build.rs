// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! s3dlio fork patch: enable the appendable-object (Rapid/zonal) API without
//! requiring downstream crates to pass `--cfg google_cloud_unstable_storage_bidi`
//! through RUSTFLAGS. A cfg emitted by a build script applies to this crate only,
//! which is sufficient: no other crate in the dependency path gates code on it.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(google_cloud_unstable_storage_bidi)");
    println!("cargo:rustc-cfg=google_cloud_unstable_storage_bidi");
}
