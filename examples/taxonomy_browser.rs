//! Export a self-contained, searchable, expanding animal tree to stdout.
//! Run: `cargo run --example taxonomy_browser --features taxonomy > animals.html`
use hodgepodge::taxonomy::{snapshot_tsv, Taxon};

fn main() {
    // Escaping '<' prevents data from terminating the script element. The UI
    // inserts all source strings through textContent, never through innerHTML.
    let data = serde_json::to_string(snapshot_tsv())
        .expect("serialize bundled taxonomy")
        .replace('<', "\\u003c");
    let page = include_str!("taxonomy_browser.html").replace("__TAXONOMY_DATA__", &data);
    eprintln!("Exported {} taxa", Taxon::count());
    println!("{page}");
}
