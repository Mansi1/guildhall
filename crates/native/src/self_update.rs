//! Guildhall updating itself.
//!
//! Delegated to [`craft_update::self_update`], with Guildhall's own [`APP`]. In-place updates are
//! off for now: `craft-update` still assumes ArtCraft's identity for every app it replaces
//! (`ai.storyteller.<id>` bundle ids, the `Learning Machines LLC` installer publisher and ArtCraft's
//! Apple Developer ID team), so it can't safely recognise or verify a Guildhall install. Until it
//! takes an app's identity as data (storytold/craft-launcher#989, step 1), [`current`] reports no
//! replaceable install and the launcher only announces a newer release, linking to its page.

use std::ffi::OsString;

use guildhall_core::platform::Platform;
use guildhall_core::protocol::JobPhase;
use guildhall_core::release::Release;
use guildhall_core::version::Version;

use crate::http::{Http, Stop};
use crate::paths::Paths;

pub use craft_update::self_update::{FINISH_FLAG, Install, PRETEND_VAR, is_self_updating};

/// Guildhall, as `craft-update` knows it: releases come only from its own repository.
pub const APP: craft_update::App = craft_update::App {
    developer: "Guildhall contributors",
    ..craft_update::App::new(
        guildhall_core::catalog::LAUNCHER.id,
        guildhall_core::catalog::LAUNCHER.name,
        guildhall_core::catalog::LAUNCHER.tagline,
        guildhall_core::catalog::LAUNCHER_REPO,
    )
};

/// The version this launcher counts as (see [`PRETEND_VAR`]).
pub fn current_version() -> Option<Version> {
    craft_update::self_update::current_version(&APP, env!("CARGO_PKG_VERSION"))
}

/// How this copy is installed, if it's one the launcher knows how to replace: never, for now (see
/// the module docs), so the launcher offers the release page instead of replacing itself.
pub fn current() -> Option<Install> {
    None
}

fn to_update_paths(paths: &Paths) -> craft_update::Paths {
    craft_update::Paths { data: paths.data.clone(), cache: paths.cache.clone() }
}

fn to_update_release(release: &Release) -> craft_update::Release {
    craft_update::Release {
        tag: release.tag.clone(),
        version: release.version.clone(),
        name: release.name.clone(),
        published_at: release.published_at,
        html_url: release.html_url.clone(),
        notes: release.notes.clone(),
        prerelease: release.prerelease,
        assets: release
            .assets
            .iter()
            .map(|a| craft_update::Asset { name: a.name.clone(), size: a.size, url: a.url.clone(), sha256: a.sha256.clone() })
            .collect(),
    }
}

fn to_update_platform(platform: Platform) -> craft_update::Platform {
    craft_update::Platform {
        os: match platform.os {
            guildhall_core::Os::Windows => craft_update::Os::Windows,
            guildhall_core::Os::MacOs => craft_update::Os::MacOs,
            guildhall_core::Os::Linux => craft_update::Os::Linux,
            guildhall_core::Os::FreeBsd => craft_update::Os::FreeBsd,
        },
        arch: match platform.arch {
            guildhall_core::Arch::X64 => craft_update::Arch::X64,
            guildhall_core::Arch::Arm64 => craft_update::Arch::Arm64,
            guildhall_core::Arch::X86 => craft_update::Arch::X86,
            guildhall_core::Arch::Riscv64 => craft_update::Arch::Riscv64,
        },
    }
}

/// Whether `release` is the version this launcher just updated itself to.
pub fn just_updated_to(paths: &Paths, release: &Release) -> bool {
    let update_paths = to_update_paths(paths);
    let update_release = to_update_release(release);
    craft_update::self_update::just_updated_to(&update_paths, &update_release)
}

/// The error of the last update, if it failed, once.
pub fn take_failure(paths: &Paths) -> Option<String> {
    let update_paths = to_update_paths(paths);
    craft_update::self_update::take_failure(&update_paths)
}

/// Downloads `release` for `platform`, puts it in place and starts the helper that relaunches
/// it once this process has exited. On success the caller must quit promptly. `last` is called
/// once the download is ready and returns when the launcher may replace itself and restart.
#[allow(clippy::too_many_arguments)]
pub fn update(
    install: &Install,
    release: &Release,
    platform: Platform,
    _http: &Http,
    paths: &Paths,
    stop: &Stop,
    phase: &mut dyn FnMut(JobPhase),
    last: &dyn Fn() -> Result<(), String>,
) -> Result<(), String> {
    let update_release = to_update_release(release);
    let update_platform = to_update_platform(platform);
    let update_paths = to_update_paths(paths);
    let update_http = craft_update::Http::new("Guildhall/SelfUpdate")?;
    let mut update_phase = |p: craft_update::JobPhase| {
        phase(match p {
            craft_update::JobPhase::Downloading { done, total } => JobPhase::Downloading { done, total },
            craft_update::JobPhase::Reconnecting { done, total } => JobPhase::Reconnecting { done, total },
            craft_update::JobPhase::Waiting => JobPhase::Waiting,
            craft_update::JobPhase::Verifying => JobPhase::Verifying,
            craft_update::JobPhase::Installing => JobPhase::Installing,
            craft_update::JobPhase::Removing => JobPhase::Removing,
        });
    };
    craft_update::self_update::update(
        &APP,
        install,
        &update_release,
        update_platform,
        &update_http,
        &update_paths,
        stop,
        &mut update_phase,
        last,
        Some(env!("CARGO_PKG_VERSION")),
    )
}

/// Finishes an update when this program was started as the update helper.
pub fn finish(args: &[OsString]) -> Option<Result<(), String>> {
    craft_update::self_update::finish(&APP, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_development_build_does_not_update_itself() {
        assert_eq!(current(), None);
    }

    #[test]
    fn updates_come_from_guildhalls_own_repository() {
        assert_eq!(APP.repo, guildhall_core::catalog::LAUNCHER_REPO);
        assert_eq!(APP.id, guildhall_core::catalog::LAUNCHER_ID);
        assert_ne!(APP.repo, craft_update::suite::LAUNCHER.repo);
    }
}
