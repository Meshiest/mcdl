// use anyhow::Result;
use clap::{clap_app, crate_version};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    fs::{remove_file, File},
    process::exit,
};

const MANIFEST_URL: &'static str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Deserialize)]
struct ManifestEntry {
    id: String,
    url: String,
}

#[derive(Deserialize)]
struct LatestVersions {
    release: String,
    snapshot: String,
}

#[derive(Deserialize)]
struct ManifestBlob {
    latest: LatestVersions,
    versions: Vec<ManifestEntry>,
}

#[derive(Deserialize)]
struct DownloadEntry {
    sha1: String,
    size: usize,
    url: String,
}

#[derive(Deserialize)]
struct VersionBlob {
    id: String,
    downloads: HashMap<String, DownloadEntry>,
}

// lazy macro for exiting with code 1
macro_rules! exit {
    ($($e:expr),+) => {
        eprintln!($($e),+);
        exit(1);
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
fn main() -> Result<()> {
    // arg parsing
    let matches = clap_app!(myapp =>
        (version: crate_version!())
        (author: "cake")
        (about: "Automatically downloads latest minecraft server as 'server.jar'")
        (@arg snapshot: --snapshot -s ... "Use the latest snapshot")
        (@arg latest: --latest -l ... "Use latest version, snapshot or not")
        (@arg print: --print -p ... "Print the version instead of downloading it")
        (@arg quiet: --quiet -q ... "Don't print any unnecessary output")
        (@arg named: --named -n ... "Use the version as the file name")
        (@arg rename: --rename -r +takes_value ... "Provide a file name (.jar) is appended")
        (@arg insecure: --insecure -i ... "Don't check the sha1 for the file")
        (@arg VERSION: +takes_value ... "Get a specific version")
    )
    .get_matches();

    let response = ureq::get(MANIFEST_URL).call()?;
    let manifest: ManifestBlob = response.into_json()?;

    let quiet = matches.is_present("quiet");

    macro_rules! log {
        ($($e:expr),+) => {
            if !quiet {
                println!($($e),+);
            }
        }
    }

    // handle args, default to latest release
    let target_version = match (
        matches.is_present("snapshot"),
        matches.is_present("latest"),
        matches.value_of("VERSION"),
    ) {
        (false, false, None) => &manifest.latest.release,
        (true, _, _) => &manifest.latest.snapshot,
        (_, true, _) if manifest.versions.len() > 0 => &manifest.versions[0].id,
        (_, _, Some(version)) => {
            if let Some(found) = manifest.versions.iter().find(|&e| e.id.eq(version)) {
                &found.id
            } else {
                exit!(
                    "error: unable to find manifest entry with version '{}'",
                    version
                );
            }
        }
        _ => &manifest.latest.release,
    };

    // find the respective version
    let target_entry = manifest
        .versions
        .iter()
        .find(|&e| e.id.eq(target_version))
        .expect("could not find entry with version obtained from the manifest");

    // print the version if the flag is present
    if matches.is_present("print") {
        println!("{}", target_version);
        return Ok(());
    }
    log!("found version {}", target_version);

    let response = ureq::get(&target_entry.url).call()?;
    let blob: VersionBlob = response.into_json()?;

    assert_eq!(
        &blob.id, target_version,
        "manifest provided incorrect url target version"
    );

    let download = if let Some(download) = blob.downloads.get("server") {
        download
    } else {
        exit!(
            "error: no server jar available for version '{}'",
            target_version
        );
    };

    // filename based on args
    let file_name = match (matches.is_present("named"), matches.value_of("rename")) {
        (true, _) => format!("server-{}", target_version),
        (_, Some(name)) => name.to_string(),
        _ => "server".to_string(),
    };

    log!("downloading {}", download.url);
    let response = ureq::get(&download.url).call()?;
    assert_eq!(
        response
            .header("content-length")
            .expect("download url did not provide content-length")
            .parse::<usize>()
            .unwrap(),
        download.size,
        "manifest provided different content length"
    );

    let file_path = format!("{}.jar", file_name);

    let mut file = File::create(&file_path)?;
    std::io::copy(&mut response.into_reader(), &mut file)?;

    log!("checking file hash {}", download.sha1);
    if !matches.is_present("insecure") {
        let mut sha1 = Sha1::new();
        let mut file = File::open(&file_path)?;
        std::io::copy(&mut file, &mut sha1)?;
        let hash = format!("{:02x}", sha1.finalize());

        if hash != download.sha1 {
            log!("invalid hash {}, removing file", hash);
            remove_file(file_path)?;
            exit!("error: downloaded jar has invalid hash");
        }
    }

    log!("successfully downloaded {}.jar", file_name);
    Ok(())
}
