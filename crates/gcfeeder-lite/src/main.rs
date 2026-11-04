//! gcfeeder-lite: a headless frontend for `gcfeeder-core`.
//!
//! No window, no GPU, no tray icon, no repaint loop. Every thread spends its
//! life blocked in the kernel (USB read / channel wait), so idle CPU use is ~0%.
//! It reads the same `gcfeeder.toml` as the full GUI version.
#![cfg_attr(feature = "background", windows_subsystem = "windows")]

mod config;

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process,
    sync::Mutex,
    thread,
    time::Instant,
};

use config::{Config, Profile};
use gcfeeder_core::{
    adapter::{poller::Poller, Port},
    feeder::Feeder,
};
use log::{info, warn, LevelFilter, Log, Metadata, Record};

type Usb = rusb::GlobalContext;

const CONFIG_FILE: &str = "gcfeeder.toml";
const LOG_FILE: &str = "gcfeeder-lite.log";

const HELP: &str = "\
gcfeeder-lite - headless, low-CPU GameCube adapter feeder

USAGE:
    gcfeeder-lite [OPTIONS]

OPTIONS:
    --ports <list>    Ports to feed, e.g. `1` or `1,2` (default: 1,2,3,4)
    --config <path>   Path to gcfeeder.toml (default: next to the executable)
    --verbose         Also log info messages to gcfeeder-lite.log
    -h, --help        Show this help
";

struct Args {
    ports: [bool; Port::COUNT],
    config_path: PathBuf,
    verbose: bool,
}

fn parse_args(exe_dir: &std::path::Path) -> Result<Args, String> {
    let mut args = Args {
        ports: [true; Port::COUNT],
        config_path: exe_dir.join(CONFIG_FILE),
        verbose: false,
    };

    let mut it = env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print!("{HELP}");
                process::exit(0);
            }
            "--verbose" => args.verbose = true,
            "--config" => {
                args.config_path = it.next().ok_or("--config needs a path")?.into();
            }
            "--ports" => {
                let list = it.next().ok_or("--ports needs a list like 1,2")?;
                args.ports = [false; Port::COUNT];
                for part in list.split(',') {
                    let n: usize = part
                        .trim()
                        .parse()
                        .map_err(|_| format!("invalid port '{part}'"))?;
                    if !(1..=Port::COUNT).contains(&n) {
                        return Err(format!("port {n} out of range 1-{}", Port::COUNT));
                    }
                    args.ports[n - 1] = true;
                }
            }
            other => return Err(format!("unknown option '{other}'")),
        }
    }

    Ok(args)
}

/// Tiny file logger: warnings and errors by default, nothing is formatted or
/// written on the hot path.
struct FileLogger {
    file: Mutex<File>,
    start: Instant,
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Ok(mut f) = self.file.lock() {
                let _ = writeln!(
                    f,
                    "[{:>9.3}s] {:<5} {}",
                    self.start.elapsed().as_secs_f64(),
                    record.level(),
                    record.args()
                );
            }
        }
    }

    fn flush(&self) {}
}

fn init_logger(exe_dir: &std::path::Path, verbose: bool) {
    if let Ok(file) = File::create(exe_dir.join(LOG_FILE)) {
        let logger = FileLogger {
            file: Mutex::new(file),
            start: Instant::now(),
        };
        if log::set_boxed_logger(Box::new(logger)).is_ok() {
            log::set_max_level(if verbose {
                LevelFilter::Info
            } else {
                LevelFilter::Warn
            });
        }
    }
}

fn load_config(path: &std::path::Path) -> Config {
    match fs::read_to_string(path) {
        Ok(s) => match toml::from_str::<Config>(&s) {
            Ok(config) => config,
            Err(e) => {
                warn!("Failed to parse {}: {e}; using defaults", path.display());
                Config::default()
            }
        },
        Err(e) => {
            warn!("Failed to read {}: {e}; using defaults", path.display());
            Config::default()
        }
    }
}

fn main() {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));

    let args = match parse_args(&exe_dir) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}\n\n{HELP}");
            process::exit(2);
        }
    };

    init_logger(&exe_dir, args.verbose);

    let config = load_config(&args.config_path);
    let poller = Poller::<Usb>::new(Usb {});

    let mut feeders: Vec<Feeder<Usb>> = Vec::new();
    for index in 0..Port::COUNT {
        if !args.ports[index] {
            continue;
        }

        let port = Port::try_from(index).expect("port index in range");
        let profile: Profile = config
            .profile
            .selected(port)
            .cloned()
            .unwrap_or_else(|| {
                warn!("Missing profile for port {port:?}, using default");
                Profile::default()
            });

        feeders.push(Feeder::new(profile, poller.add_listener(port)));
    }

    info!("Started with {} port(s)", feeders.len());
    println!(
        "gcfeeder-lite running on {} port(s). Press Ctrl+C or close this window to quit.",
        feeders.len()
    );

    // Everything runs on worker threads that block in the kernel; the main
    // thread just sleeps forever (park, no polling, no timers).
    loop {
        thread::park();
    }
}
