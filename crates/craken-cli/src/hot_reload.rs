use std::{path::Path, time::Duration};

use anyhow::Result;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

/// Run the development server with automatic hot-reload.
///
/// # Behaviour
///
/// 1. Spawns `cargo run -- serve --addr <addr>` as a child process.
/// 2. Watches `./src` for `Create`, `Modify`, or `Remove` events via
///    the [`notify`] crate.
/// 3. On change: kills the running child, waits for it to exit, drains
///    any burst of additional events (300 ms debounce), and re-spawns.
/// 4. If the child exits on its own (e.g. compile error), the loop waits
///    for the next file-change event before attempting to restart, avoiding
///    a tight CPU-burning restart loop.
///
/// # Limitations
///
/// Because Rust is AOT-compiled, "hot reload" here means recompilation and
/// process restart — not in-process code swapping. Use `RUST_LOG=info` for
/// structured log output during development.
pub async fn run_dev(addr: &str) -> Result<()> {
    tracing::info!("Craken dev server — hot-reload enabled");
    tracing::info!("Addr: {addr}");

    // Bridge notify's sync callback into an async tokio channel.
    let (tx, mut rx) = mpsc::channel::<()>(64);

    let tx_for_watcher = tx.clone();
    let mut watcher = RecommendedWatcher::new(
        move |result: notify::Result<notify::Event>| {
            if let Ok(event) = result {
                // Only react to meaningful filesystem mutations.
                if matches!(
                    event.kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                ) {
                    // `blocking_send` is safe here: notify callbacks run on a
                    // plain OS thread, outside the tokio runtime.
                    let _ = tx_for_watcher.blocking_send(());
                }
            }
        },
        // Poll every 500 ms as a fallback on platforms where inotify is
        // unavailable or has limitations (e.g. network filesystems).
        Config::default().with_poll_interval(Duration::from_millis(500)),
    )?;

    let src = Path::new("src");
    if src.exists() {
        watcher.watch(src, RecursiveMode::Recursive)?;
        tracing::info!("Watching: src/");
    } else {
        tracing::warn!("src/ not found — file watching disabled");
    }

    loop {
        tracing::info!("Spawning: cargo run -- serve {addr}");

        let mut child = tokio::process::Command::new("cargo")
            .args(["run", "--", "serve", addr])
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn cargo: {e}"))?;

        tokio::select! {
            // ── Branch 1: child process exited on its own ──────────────────
            status = child.wait() => {
                match status {
                    Ok(s) if s.success() => tracing::info!("Server exited cleanly"),
                    Ok(s)  => tracing::warn!("Server exited: {s}"),
                    Err(e) => tracing::error!("Server wait error: {e}"),
                }
                // Wait for a file change before re-spawning to avoid hammering
                // the filesystem with rebuild attempts on a compile error.
                tracing::info!("Waiting for file change…");
                rx.recv().await;
                // Debounce burst events.
                while rx.try_recv().is_ok() {}
                tokio::time::sleep(Duration::from_millis(300)).await;
            }

            // ── Branch 2: file change detected ────────────────────────────
            Some(()) = rx.recv() => {
                tracing::info!("File change detected — restarting server");
                let _ = child.kill().await;
                let _ = child.wait().await;
                // Drain any burst of change events before re-spawning.
                while rx.try_recv().is_ok() {}
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        }
    }
}
