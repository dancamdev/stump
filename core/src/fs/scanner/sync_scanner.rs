use std::{
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc,
	},
	time::Duration,
};
use tracing::{debug, error, trace, warn};

use crate::{
	db::models::LibraryOptions,
	event::CoreEvent,
	fs::scanner::{
		setup::{setup_series, SeriesSetup},
		utils, ScannedFileTrait,
	},
	job::{persist_job_start, runner::RunnerCtx, JobUpdate},
	prelude::{CoreResult, Ctx},
	prisma::series,
};

use super::setup::{setup_library, LibrarySetup};

async fn scan_series(
	ctx: Ctx,
	runner_id: String,
	series: series::Data,
	library_path: &str,
	library_options: LibraryOptions,
	mut on_progress: impl FnMut(String) + Send + Sync + 'static,
) {
	debug!(?series, "Scanning series");
	let SeriesSetup {
		mut visited_media,
		media_by_path,
		walkdir,
		glob_set,
	} = setup_series(&ctx, &series, library_path, &library_options).await;

	for entry in walkdir
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.path().is_file())
	{
		let path = entry.path();
		let path_str = path.to_str().unwrap_or("");
		trace!(?path, "Scanning file");

		// Tell client we are on the next file, this will increment the counter in the
		// callback, as well.
		on_progress(format!("Analyzing {:?}", path));

		let glob_match = glob_set.is_match(path);

		if path.should_ignore() || glob_match {
			trace!(?path, glob_match, "Skipping ignored file");
			continue;
		} else if visited_media.get(path_str).is_some() {
			trace!(media_path = ?path, "Existing media found");
			let media = media_by_path.get(path_str).unwrap();
			let check_modified_at =
				utils::file_updated_since_scan(&entry, media.modified_at.clone());

			if let Ok(has_been_modified) = check_modified_at {
				// If the file has been modified since the last scan, we need to update it.
				if has_been_modified {
					debug!(?media, "Media file has been modified since last scan");
					// TODO: do something with media_updates
					warn!(
						outdated_media = ?media,
						"Stump does not support updating media entities yet",
					);
				}
			}

			*visited_media.entry(path_str.to_string()).or_insert(true) = true;
			continue;
		}

		debug!(series_id = ?series.id, new_media_path = ?path, "New media found in series");
		match utils::insert_media(&ctx, path, series.id.clone(), &library_options).await {
			Ok(media) => {
				visited_media.insert(media.path.clone(), true);
				ctx.emit_client_event(CoreEvent::CreatedMedia(Box::new(media)));
			},
			Err(e) => {
				error!(error = ?e, "Failed to create media");
				ctx.handle_failure_event(CoreEvent::CreateEntityFailed {
					runner_id: Some(runner_id.clone()),
					path: path.to_str().unwrap_or_default().to_string(),
					message: e.to_string(),
				})
				.await;
			},
		}
	}

	let missing_media = visited_media
		.into_iter()
		.filter(|(_, visited)| !visited)
		.map(|(path, _)| path)
		.collect::<Vec<String>>();

	if missing_media.is_empty() {
		warn!(
			missing_paths = ?missing_media,
			"Some media files could not be located during scan."
		);
		let result = utils::mark_media_missing(&ctx, missing_media).await;

		if let Err(err) = result {
			error!(error = ?err, "Failed to mark media as MISSING");
		} else {
			debug!("Marked {} media as MISSING", result.unwrap());
		}
	}
}

pub async fn scan(ctx: RunnerCtx, path: String, runner_id: String) -> CoreResult<u64> {
	let core_ctx = ctx.core_ctx.clone();

	let LibrarySetup {
		library,
		library_options,
		library_series,
		tasks,
	} = setup_library(&core_ctx, path).await?;

	// Sleep for a little to let the UI breathe.
	tokio::time::sleep(Duration::from_millis(700)).await;

	// TODO: I am not sure if jobs should fail when the job fails to persist to DB.
	let _job = persist_job_start(&core_ctx, runner_id.clone(), tasks).await?;

	ctx.progress(JobUpdate::job_started(
		runner_id.clone(),
		0,
		tasks,
		Some(format!("Starting library scan at {}", &library.path)),
	));

	let counter = Arc::new(AtomicU64::new(0));

	for series in library_series {
		let progress_ctx = ctx.clone();
		let scanner_ctx = core_ctx.clone();
		let r_id = runner_id.clone();

		let counter_ref = counter.clone();
		let runner_id = runner_id.clone();
		let library_path = library.path.clone();
		// Note: I don't ~love~ having to clone this struct each iteration. I think it's fine for now,
		// considering it consists of just a few booleans.
		let library_options = library_options.clone();

		scan_series(
			scanner_ctx,
			runner_id,
			series,
			&library_path,
			library_options,
			move |msg| {
				let previous = counter_ref.fetch_add(1, Ordering::SeqCst);

				progress_ctx.progress(JobUpdate::job_progress(
					r_id.to_owned(),
					Some(previous + 1),
					tasks,
					Some(msg),
				));
			},
		)
		.await;
	}

	ctx.progress(JobUpdate::job_finishing(
		runner_id,
		Some(counter.load(Ordering::SeqCst)),
		tasks,
		None,
	));
	tokio::time::sleep(Duration::from_millis(1000)).await;

	Ok(counter.load(Ordering::SeqCst))
}
