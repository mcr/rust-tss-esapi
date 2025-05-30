// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use semver::{Version, VersionReq, Prerelease};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(hierarchy_is_esys_tr)");
    println!("cargo:rustc-check-cfg=cfg(has_tss_base_rc_values_28_to_51)");
    println!("cargo:rustc-check-cfg=cfg(has_tss_base_rc_values_52_to_53)");
    println!("cargo:rustc-check-cfg=cfg(has_tpmu_sensitive_create)");
    println!("cargo:rustc-check-cfg=cfg(has_esys_tr_get_tpm_handle)");

    let tss_version_string = std::env::var("DEP_TSS2_ESYS_VERSION")
        .expect("Failed to parse ENV variable DEP_TSS2_ESYS_VERSION as string");

    let mut tss_version = Version::parse(&tss_version_string)
        .expect("Failed to parse the DEP_TSS2_ESYS_VERSION variable as a semver version");

    // nuke any prerelease info, which probably is just a git repo/dirty flag
    // like: 4.0.1-67-gb7bad346
    tss_version.pre = Prerelease::EMPTY;
    
    //eprintln!("tss version: {} / {:?}", tss_version_string, tss_version);
    let supported_tss_version =
        VersionReq::parse("<5.0.0, >=2.3.3").expect("Failed to parse supported TSS version");

    assert!(
        supported_tss_version.matches(&tss_version),
        "Unsupported TSS version {}",
        tss_version
    );

    let hierarchy_is_esys_tr_req = VersionReq::parse(">=3.0.0").unwrap();
    if hierarchy_is_esys_tr_req.matches(&tss_version) {
        println!("cargo:rustc-cfg=hierarchy_is_esys_tr")
    }

    let has_tss_base_rc_values_28_to_51_req = VersionReq::parse(">=2.4.0").unwrap();
    if has_tss_base_rc_values_28_to_51_req.matches(&tss_version) {
        println!("cargo:rustc-cfg=has_tss_base_rc_values_28_to_51")
    }

    let has_tss_base_rc_values_52_to_53_req = VersionReq::parse(">=3.0.0").unwrap();
    if has_tss_base_rc_values_52_to_53_req.matches(&tss_version) {
        println!("cargo:rustc-cfg=has_tss_base_rc_values_52_to_53")
    }

    let has_tpmu_sensitive_create_req = VersionReq::parse(">=4.0.0").unwrap();
    if has_tpmu_sensitive_create_req.matches(&tss_version) {
        println!("cargo:rustc-cfg=has_tpmu_sensitive_create")
    }

    #[cfg(feature = "generate-bindings")]
    {
        let has_esys_tr_get_tpm_handle_req = VersionReq::parse(">=2.4.0").unwrap();
        if has_esys_tr_get_tpm_handle_req.matches(&tss_version) {
            println!("cargo:rustc-cfg=has_esys_tr_get_tpm_handle")
        }
    }
}
