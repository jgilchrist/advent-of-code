// Used to be able to compare runtime-generated String results with
// static strings embedded in the binary. There's likely a better
// way to do this, but I haven't found it.
// This is fine since the process lives only long enough to run the solution.
pub fn leak_string_to_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
