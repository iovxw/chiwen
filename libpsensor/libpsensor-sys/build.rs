extern crate gcc;
extern crate pkg_config;

static LIBPSENSOR_SRC: &str = "psensor-1.2.0/src/lib/";
static LIBPSENSOR_SOURCES: [&str; 14] = [
    "color.c",
    "hdd_hddtemp.c",
    "measure.c",
    "plog.c",
    "pmutex.c",
    "psensor.c",
    "ptime.c",
    "pio.c",
    "slog.c",
    "temperature.c",
    "url.c",
    // OPTION
    "lmsensor.c",
    "nvidia.c",
    "pudisks2.c",
];

fn main() {
    let mut config = gcc::Build::new();
    for source in &LIBPSENSOR_SOURCES {
        config.file(format!("{}{}", LIBPSENSOR_SRC, source));
    }
    // pudisks2.c
    let udisks2 = pkg_config::Config::new()
        .cargo_metadata(false)
        .atleast_version("2.0.0")
        .probe("udisks2")
        .unwrap();
    for inc in udisks2.include_paths {
        config.include(inc);
    }
    config
        .warnings(false)
        .file("wrapper.c")
        .include(".")
        .include(LIBPSENSOR_SRC)
        .compile("libpsensor.a");
    // lmsensor.c
    println!("cargo:rustc-link-lib=sensors");
    // nvidia.c
    println!("cargo:rustc-link-lib=XNVCtrl");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xext");
    // pudisks2.c
    for link in udisks2.libs {
        println!("cargo:rustc-link-lib={}", link);
    }
}
