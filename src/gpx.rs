use anyhow::Context;
use geo_types::geometry::Point;
use gpx::Gpx;
use std::{
	collections::BTreeMap,
	fs::File,
	io::BufReader,
	path::{Path, PathBuf}
};
use time::OffsetDateTime;

type Tree = BTreeMap<OffsetDateTime, Point<f64>>;

/// Load the waypoints of all tracks.
/// Return the resulting tree and the count of skipped tracks.
pub fn load_tracks(files: &Vec<PathBuf>) -> anyhow::Result<(Tree, usize)> {
	let mut tree = Tree::new();
	let mut skipped: usize = 0;
	for file in files {
		skipped += load_track(&mut tree, file)?;
	}
	Ok((tree, skipped))
}

/// Load one track waypoints to tree. Return the count of skipped tracks
fn load_track(tree: &mut Tree, file: &Path) -> anyhow::Result<usize> {
	println!("load gpx track {file:?}");
	let file = File::open(file).with_context(|| format!("failed to open {file:?}"))?;
	let reader = BufReader::new(&file);
	let gpx: Gpx =
		gpx::read(reader).with_context(|| format!("failed to prase {file:?}"))?;
	let mut no_time_stamps_count: usize = 0;
	for waypoint in gpx
		.tracks
		.iter()
		.map(|track| track.segments.iter())
		.flatten()
		.map(|segment| segment.points.iter())
		.flatten()
	{
		if let Some(time) = waypoint.time {
			tree.insert(time.into(), waypoint.point());
		} else {
			no_time_stamps_count += 1;
		}
	}
	if no_time_stamps_count != 0 {
		println!("Warning: {file:?} inculdes {no_time_stamps_count} waypoints without timestamp");
	}
	Ok(no_time_stamps_count)
}
