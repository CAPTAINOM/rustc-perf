use cargotest::rustc_host;
use cargotest::support::registry::Package;
use cargotest::support::{cross_compile, execs, project};
use hamcrest::assert_that;

#[test]
fn no_deps() {
    let p = project("foo")
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            authors = []
            version = "0.0.1"
        "#,
        )
        .file(
            "src/main.rs",
            r#"
            mod a; fn main() {}
        "#,
        )
        .file("src/a.rs", "")
        .build();

    assert_that(p.cargo("fetch"), execs().with_status(0).with_stdout(""));
}

#[test]
fn fetch_all_platform_dependencies_when_no_target_is_given() {
    Package::new("d1", "1.2.3")
        .file(
            "Cargo.toml",
            r#"
            [project]
            name = "d1"
            version = "1.2.3"
        "#,
        )
        .file("src/lib.rs", "")
        .publish();

    Package::new("d2", "0.1.2")
        .file(
            "Cargo.toml",
            r#"
            [project]
            name = "d2"
            version = "0.1.2"
        "#,
        )
        .file("src/lib.rs", "")
        .publish();

    let target = cross_compile::alternate();
    let host = rustc_host();
    let p = project("foo")
        .file(
            "Cargo.toml",
            &format!(
                r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [target.{host}.dependencies]
            d1 = "1.2.3"

            [target.{target}.dependencies]
            d2 = "0.1.2"
        "#,
                host = host,
                target = target
            ),
        )
        .file("src/lib.rs", "")
        .build();

    assert_that(
        p.cargo("fetch"),
        execs()
            .with_status(0)
            .with_stderr_contains("[..] Downloading d1 v1.2.3 [..]")
            .with_stderr_contains("[..] Downloading d2 v0.1.2 [..]"),
    );
}

#[test]
fn fetch_platform_specific_dependencies() {
    Package::new("d1", "1.2.3")
        .file(
            "Cargo.toml",
            r#"
            [project]
            name = "d1"
            version = "1.2.3"
        "#,
        )
        .file("src/lib.rs", "")
        .publish();

    Package::new("d2", "0.1.2")
        .file(
            "Cargo.toml",
            r#"
            [project]
            name = "d2"
            version = "0.1.2"
        "#,
        )
        .file("src/lib.rs", "")
        .publish();

    let target = cross_compile::alternate();
    let host = rustc_host();
    let p = project("foo")
        .file(
            "Cargo.toml",
            &format!(
                r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [target.{host}.dependencies]
            d1 = "1.2.3"

            [target.{target}.dependencies]
            d2 = "0.1.2"
        "#,
                host = host,
                target = target
            ),
        )
        .file("src/lib.rs", "")
        .build();

    assert_that(
        p.cargo("fetch").arg("--target").arg(&host),
        execs()
            .with_status(0)
            .with_stderr_contains("[..] Downloading d1 v1.2.3 [..]")
            .with_stderr_does_not_contain("[..] Downloading d2 v0.1.2 [..]"),
    );

    assert_that(
        p.cargo("fetch").arg("--target").arg(&target),
        execs()
            .with_status(0)
            .with_stderr_contains("[..] Downloading d2 v0.1.2[..]")
            .with_stderr_does_not_contain("[..] Downloading d1 v1.2.3 [..]"),
    );
}
