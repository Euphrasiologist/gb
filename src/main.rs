use anyhow::{bail, Result};
use dashmap::DashMap;

mod arg;

// the main function calls the rest.
fn main() -> Result<()> {
    let args = arg::parse();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(args.delimiter as u8)
        .from_reader(args.input);

    let header_set = DashMap::new();

    // get the headers and place in a HashMap, as we need the indexes
    // later on in the record iterator.
    let headers = rdr.headers()?.clone();
    for (index, header) in headers.iter().enumerate() {
        header_set.insert(header.to_string(), index);
    }

    // check that field and keys are in the headers.
    let contains_field = header_set.get(args.field.as_str()).is_some();
    let contains_keys = args
        .keys
        .iter()
        .all(|e| header_set.get(e.as_str()).is_some());

    // for now we won't distriminate between fields & keys
    if !contains_field || !contains_keys {
        bail!("Either the field or the key(s) were not recognised, please check spelling.")
    }
    let field_index = *header_set.get(args.field.as_str()).unwrap().value();
    let key_indices: Vec<usize> = args
        .keys
        .iter()
        .map(|e| *header_set.get(e.as_str()).unwrap().value())
        .collect();

    // load the whole structure into a dashmap for now anyway.
    // might optimise this later.
    let map = DashMap::new();

    for result in rdr.records() {
        let record = result?;
        // get the field
        let field: String = record[field_index].to_string();
        let field = field.parse::<f64>();

        let mut no_rows_skipped = 0;

        let parsed_field = match field {
            Ok(f) => f,
            Err(_) => {
                no_rows_skipped += 1;
                continue;
            }
        };

        let keys: Vec<String> = record
            .iter()
            .enumerate()
            .filter(|(i, _)| key_indices.iter().any(|ii| i == ii))
            .map(|(_, e)| e.to_string())
            .collect();

        map.entry(keys).or_insert(Vec::new()).push(parsed_field);
    }

    println!("{}\tN", args.keys.join(&args.delimiter.to_string()));
    for (k, v) in map {
        let summary = arg::Summary::inner_fn(&args.function, &v);
        println!("{}\t{:?}", k.join(&args.delimiter.to_string()), summary)
    }

    Ok(())
}
