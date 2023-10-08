use clap::Parser;
use std::path::PathBuf;

mod gpx;
use gpx::load_tracks;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opt {
	/// gpx tracks, wich will be used as source for the location.
	// Waypoints in tracks must have a time stamp.
	#[arg(short, long)]
	tracks: Vec<PathBuf>
}

fn main() {
	let opt = Opt::parse();
	let (points, skipped_points) =
		load_tracks(&opt.tracks).expect("failed to load gpx tracks");
	println!("{points:#?}");
	println!("loaded {} way points", points.len());
	println!("skipped {} way points", skipped_points);
}
