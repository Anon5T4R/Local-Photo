//! Give memory the system allocator keeps after frees back to the system
//! ([`localphoto_engine::memory::set_release_hook`]). The platform call lives in
//! `localphoto-sysmem`, the workspace's only crate allowed `unsafe`.

/// Install the hook.
pub fn install() {
    localphoto_engine::memory::set_release_hook(release);
}

fn release() {
    let _ = localphoto_sysmem::release_free_memory();
}
