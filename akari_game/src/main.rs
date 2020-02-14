#[cfg(feature = "profiler")]
#[macro_use]
extern crate thread_profiler;

fn main() -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    thread_profiler::register_thread_with_profiler();

    {
        #[cfg(feature = "profiler")]
        profile_scope!("run");
        akari_core::run()?;
    }

    #[cfg(feature = "profiler")]
    thread_profiler::write_profile("profile.json");

    Ok(())
}
