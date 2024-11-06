use sysinfo::System;

#[allow(dead_code)]
pub fn get_system_info() {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", System::name().unwrap());
    println!(
        "System kernel version:   {:?}",
        System::kernel_version().unwrap()
    );
    println!(
        "System OS version:       {:?}",
        System::os_version().unwrap()
    );
    println!(
        "System host name:        {:?}",
        System::host_name().unwrap()
    );

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
}
