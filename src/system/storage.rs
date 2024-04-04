// We display all disks' information:
let disks = Disks::new_with_refreshed_list();
for disk in &disks {
    println!("{disk:?}");
}