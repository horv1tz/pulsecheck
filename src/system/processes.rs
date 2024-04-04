// Display processes ID, name na disk usage:
for (pid, process) in sys.processes() {
    println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
}