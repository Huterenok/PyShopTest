mod cli;

use cli::Args;
use test_3::find_hashes;

fn main() {
    let args = Args::receive();
    find_hashes(args.n, args.f)
        .iter()
        .for_each(|(x, hash)| println!("{}, {}", x, hash));
}
