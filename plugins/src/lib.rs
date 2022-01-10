use gst::glib;
use once_cell::sync::Lazy;

mod signaller;
pub mod webrtcsink;

pub static RUNTIME_HANDLE: Lazy<tokio::runtime::Handle> = Lazy::new(|| {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle
    } else {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.handle().clone()
    }
});

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    webrtcsink::register(plugin)?;

    Ok(())
}

gst::plugin_define!(
    webrtcsink,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
