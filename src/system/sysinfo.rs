// Display system information:
println!("System name:             {:?}", System::name());
println!("System kernel version:   {:?}", System::kernel_version());
println!("System OS version:       {:?}", System::os_version());
println!("System host name:        {:?}", System::host_name());

// Components temperature:
let mut components = Components::new_with_refreshed_list();
loop {
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    let term = console::Term::stdout();
    term.clear_screen().expect("Не удалось очистить консоль");
    components.refresh();