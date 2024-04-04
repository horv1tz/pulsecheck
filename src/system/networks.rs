// Network interfaces name, total data received and total data transmitted:
let networks = Networks::new_with_refreshed_list();
println!("=> networks:");
for (interface_name, data) in &networks {
    println!(
        "{interface_name}: {} B (down) / {} B (up)",
        data.total_received(),
        data.total_transmitted(),
    );
    // If you want the amount of data received/transmitted since last call
    // to `Networks::refresh`, use `received`/`transmitted`.
}