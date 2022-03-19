pub fn split_port_and_address(port_and_address:&str) -> Option<[&str; 2]> {

    // Split it into two
    let split_port_and_address = port_and_address.split(":");
    let split_items: Vec<&str> = split_port_and_address.collect();

    // More or less than two args
    if split_items.len() != 2 { return None; }

    // Return the two bits
    return Some([ split_items[0], split_items[1] ]);

}