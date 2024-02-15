// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#![cfg(unix)]

use spice_client_glib_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["spice-client-glib-2.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {cmd:?} returned {}", out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!("Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",);
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {abi_cmd:?} failed, {output:?}").into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "SpiceAudio",
        Layout {
            size: size_of::<SpiceAudio>(),
            alignment: align_of::<SpiceAudio>(),
        },
    ),
    (
        "SpiceAudioClass",
        Layout {
            size: size_of::<SpiceAudioClass>(),
            alignment: align_of::<SpiceAudioClass>(),
        },
    ),
    (
        "SpiceChannel",
        Layout {
            size: size_of::<SpiceChannel>(),
            alignment: align_of::<SpiceChannel>(),
        },
    ),
    (
        "SpiceChannelClass",
        Layout {
            size: size_of::<SpiceChannelClass>(),
            alignment: align_of::<SpiceChannelClass>(),
        },
    ),
    (
        "SpiceChannelEvent",
        Layout {
            size: size_of::<SpiceChannelEvent>(),
            alignment: align_of::<SpiceChannelEvent>(),
        },
    ),
    (
        "SpiceClientError",
        Layout {
            size: size_of::<SpiceClientError>(),
            alignment: align_of::<SpiceClientError>(),
        },
    ),
    (
        "SpiceCursorChannel",
        Layout {
            size: size_of::<SpiceCursorChannel>(),
            alignment: align_of::<SpiceCursorChannel>(),
        },
    ),
    (
        "SpiceCursorChannelClass",
        Layout {
            size: size_of::<SpiceCursorChannelClass>(),
            alignment: align_of::<SpiceCursorChannelClass>(),
        },
    ),
    (
        "SpiceCursorShape",
        Layout {
            size: size_of::<SpiceCursorShape>(),
            alignment: align_of::<SpiceCursorShape>(),
        },
    ),
    (
        "SpiceDisplayChannel",
        Layout {
            size: size_of::<SpiceDisplayChannel>(),
            alignment: align_of::<SpiceDisplayChannel>(),
        },
    ),
    (
        "SpiceDisplayChannelClass",
        Layout {
            size: size_of::<SpiceDisplayChannelClass>(),
            alignment: align_of::<SpiceDisplayChannelClass>(),
        },
    ),
    (
        "SpiceDisplayMonitorConfig",
        Layout {
            size: size_of::<SpiceDisplayMonitorConfig>(),
            alignment: align_of::<SpiceDisplayMonitorConfig>(),
        },
    ),
    (
        "SpiceDisplayPrimary",
        Layout {
            size: size_of::<SpiceDisplayPrimary>(),
            alignment: align_of::<SpiceDisplayPrimary>(),
        },
    ),
    (
        "SpiceGlScanout",
        Layout {
            size: size_of::<SpiceGlScanout>(),
            alignment: align_of::<SpiceGlScanout>(),
        },
    ),
    (
        "SpiceInputsChannel",
        Layout {
            size: size_of::<SpiceInputsChannel>(),
            alignment: align_of::<SpiceInputsChannel>(),
        },
    ),
    (
        "SpiceInputsChannelClass",
        Layout {
            size: size_of::<SpiceInputsChannelClass>(),
            alignment: align_of::<SpiceInputsChannelClass>(),
        },
    ),
    (
        "SpiceInputsLock",
        Layout {
            size: size_of::<SpiceInputsLock>(),
            alignment: align_of::<SpiceInputsLock>(),
        },
    ),
    (
        "SpiceMainChannel",
        Layout {
            size: size_of::<SpiceMainChannel>(),
            alignment: align_of::<SpiceMainChannel>(),
        },
    ),
    (
        "SpiceMainChannelClass",
        Layout {
            size: size_of::<SpiceMainChannelClass>(),
            alignment: align_of::<SpiceMainChannelClass>(),
        },
    ),
    (
        "SpicePlaybackChannel",
        Layout {
            size: size_of::<SpicePlaybackChannel>(),
            alignment: align_of::<SpicePlaybackChannel>(),
        },
    ),
    (
        "SpicePlaybackChannelClass",
        Layout {
            size: size_of::<SpicePlaybackChannelClass>(),
            alignment: align_of::<SpicePlaybackChannelClass>(),
        },
    ),
    (
        "SpicePortChannel",
        Layout {
            size: size_of::<SpicePortChannel>(),
            alignment: align_of::<SpicePortChannel>(),
        },
    ),
    (
        "SpicePortChannelClass",
        Layout {
            size: size_of::<SpicePortChannelClass>(),
            alignment: align_of::<SpicePortChannelClass>(),
        },
    ),
    (
        "SpiceQmpPortVmAction",
        Layout {
            size: size_of::<SpiceQmpPortVmAction>(),
            alignment: align_of::<SpiceQmpPortVmAction>(),
        },
    ),
    (
        "SpiceQmpStatus",
        Layout {
            size: size_of::<SpiceQmpStatus>(),
            alignment: align_of::<SpiceQmpStatus>(),
        },
    ),
    (
        "SpiceRecordChannel",
        Layout {
            size: size_of::<SpiceRecordChannel>(),
            alignment: align_of::<SpiceRecordChannel>(),
        },
    ),
    (
        "SpiceRecordChannelClass",
        Layout {
            size: size_of::<SpiceRecordChannelClass>(),
            alignment: align_of::<SpiceRecordChannelClass>(),
        },
    ),
    (
        "SpiceSession",
        Layout {
            size: size_of::<SpiceSession>(),
            alignment: align_of::<SpiceSession>(),
        },
    ),
    (
        "SpiceSessionClass",
        Layout {
            size: size_of::<SpiceSessionClass>(),
            alignment: align_of::<SpiceSessionClass>(),
        },
    ),
    (
        "SpiceSessionMigration",
        Layout {
            size: size_of::<SpiceSessionMigration>(),
            alignment: align_of::<SpiceSessionMigration>(),
        },
    ),
    (
        "SpiceSessionVerify",
        Layout {
            size: size_of::<SpiceSessionVerify>(),
            alignment: align_of::<SpiceSessionVerify>(),
        },
    ),
    (
        "SpiceSmartcardChannel",
        Layout {
            size: size_of::<SpiceSmartcardChannel>(),
            alignment: align_of::<SpiceSmartcardChannel>(),
        },
    ),
    (
        "SpiceSmartcardChannelClass",
        Layout {
            size: size_of::<SpiceSmartcardChannelClass>(),
            alignment: align_of::<SpiceSmartcardChannelClass>(),
        },
    ),
    (
        "SpiceSmartcardManager",
        Layout {
            size: size_of::<SpiceSmartcardManager>(),
            alignment: align_of::<SpiceSmartcardManager>(),
        },
    ),
    (
        "SpiceSmartcardManagerClass",
        Layout {
            size: size_of::<SpiceSmartcardManagerClass>(),
            alignment: align_of::<SpiceSmartcardManagerClass>(),
        },
    ),
    (
        "SpiceUsbDeviceManager",
        Layout {
            size: size_of::<SpiceUsbDeviceManager>(),
            alignment: align_of::<SpiceUsbDeviceManager>(),
        },
    ),
    (
        "SpiceUsbDeviceManagerClass",
        Layout {
            size: size_of::<SpiceUsbDeviceManagerClass>(),
            alignment: align_of::<SpiceUsbDeviceManagerClass>(),
        },
    ),
    (
        "SpiceUsbredirChannel",
        Layout {
            size: size_of::<SpiceUsbredirChannel>(),
            alignment: align_of::<SpiceUsbredirChannel>(),
        },
    ),
    (
        "SpiceUsbredirChannelClass",
        Layout {
            size: size_of::<SpiceUsbredirChannelClass>(),
            alignment: align_of::<SpiceUsbredirChannelClass>(),
        },
    ),
    (
        "SpiceWebdavChannel",
        Layout {
            size: size_of::<SpiceWebdavChannel>(),
            alignment: align_of::<SpiceWebdavChannel>(),
        },
    ),
    (
        "SpiceWebdavChannelClass",
        Layout {
            size: size_of::<SpiceWebdavChannelClass>(),
            alignment: align_of::<SpiceWebdavChannelClass>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) SPICE_CHANNEL_CLOSED", "12"),
    ("(gint) SPICE_CHANNEL_ERROR_AUTH", "23"),
    ("(gint) SPICE_CHANNEL_ERROR_CONNECT", "20"),
    ("(gint) SPICE_CHANNEL_ERROR_IO", "24"),
    ("(gint) SPICE_CHANNEL_ERROR_LINK", "22"),
    ("(gint) SPICE_CHANNEL_ERROR_TLS", "21"),
    ("(gint) SPICE_CHANNEL_NONE", "0"),
    ("(gint) SPICE_CHANNEL_OPENED", "10"),
    ("(gint) SPICE_CHANNEL_SWITCHING", "11"),
    ("(gint) SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD", "3"),
    (
        "(gint) SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD_AND_USERNAME",
        "5",
    ),
    ("(gint) SPICE_CLIENT_ERROR_AUTH_NEEDS_USERNAME", "4"),
    ("(gint) SPICE_CLIENT_ERROR_FAILED", "0"),
    ("(gint) SPICE_CLIENT_ERROR_USB_DEVICE_LOST", "2"),
    ("(gint) SPICE_CLIENT_ERROR_USB_DEVICE_REJECTED", "1"),
    ("(gint) SPICE_CLIENT_ERROR_USB_SERVICE", "6"),
    ("SPICE_GTK_MAJOR_VERSION", "0"),
    ("SPICE_GTK_MICRO_VERSION", "8"),
    ("SPICE_GTK_MINOR_VERSION", "39"),
    ("(guint) SPICE_INPUTS_CAPS_LOCK", "4"),
    ("(guint) SPICE_INPUTS_NUM_LOCK", "2"),
    ("(guint) SPICE_INPUTS_SCROLL_LOCK", "1"),
    ("(gint) SPICE_QMP_PORT_VM_ACTION_CONTINUE", "4"),
    ("(gint) SPICE_QMP_PORT_VM_ACTION_LAST", "5"),
    ("(gint) SPICE_QMP_PORT_VM_ACTION_PAUSE", "3"),
    ("(gint) SPICE_QMP_PORT_VM_ACTION_POWER_DOWN", "2"),
    ("(gint) SPICE_QMP_PORT_VM_ACTION_QUIT", "0"),
    ("(gint) SPICE_QMP_PORT_VM_ACTION_RESET", "1"),
    ("(gint) SPICE_SESSION_MIGRATION_CONNECTING", "3"),
    ("(gint) SPICE_SESSION_MIGRATION_MIGRATING", "2"),
    ("(gint) SPICE_SESSION_MIGRATION_NONE", "0"),
    ("(gint) SPICE_SESSION_MIGRATION_SWITCHING", "1"),
    ("(guint) SPICE_SESSION_VERIFY_HOSTNAME", "2"),
    ("(guint) SPICE_SESSION_VERIFY_PUBKEY", "1"),
    ("(guint) SPICE_SESSION_VERIFY_SUBJECT", "4"),
    ("SPICE_WEBDAV_CLIPBOARD_FOLDER_PATH", "/.spice-clipboard"),
];