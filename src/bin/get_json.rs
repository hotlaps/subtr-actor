use std::path::PathBuf;
use subtr_actor::ReplayDataCollector;

fn main() -> anyhow::Result<()> {
    // Get the replay file path from command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let filepath = PathBuf::from(&args[1]);
        let json = get_json(filepath)?;
    
        // Print the JSON to stdout
        println!("{}", json);
    } else {
        // Use default replay file path
        let filepath = PathBuf::from("replay(1).replay");
        let json = get_json(filepath)?;
        
        // Write the JSON to a file
        std::fs::write("replay_data.json", json)?;
    }

    Ok(())
}

fn get_json(filepath: PathBuf) -> anyhow::Result<String> {
    let data = std::fs::read(filepath.as_path())?;
    let replay = boxcars::ParserBuilder::new(&data)
        .must_parse_network_data()
        .on_error_check_crc()
        .parse()?;
    Ok(ReplayDataCollector::new()
        .get_replay_data(&replay)
        .map_err(|e| e.variant)?
        .as_json()?)
}