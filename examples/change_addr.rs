use std::net::Ipv4Addr;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::time::Duration;
use tokio_tun::Tun;

#[tokio::main]
async fn main() -> Result<(), tokio_tun::Error> {
    let tun = Arc::new(
        Tun::builder()
            .name("")
            .tap(false)
            .packet_info(false)
            .mtu(1350)
            .up()
            .address(Ipv4Addr::new(10, 0, 0, 1))
            .destination(Ipv4Addr::new(10, 1, 0, 1))
            .broadcast(Ipv4Addr::BROADCAST)
            .netmask(Ipv4Addr::new(255, 255, 255, 0))
            .try_build()
            .unwrap(),
    );

    println!("-----------");
    println!("tun created");
    println!("-----------");

    println!(
        "┌ name: {} ({})\n├ fd: {}\n├ mtu: {}\n├ flags: {}\n├ address: {}\n├ destination: {}\n├ broadcast: {}\n└ netmask: {}",
        tun.name(),
        tun.ifidx().unwrap(),
        tun.as_raw_fd(),
        tun.mtu().unwrap(),
        tun.flags().unwrap(),
        tun.address().unwrap(),
        tun.destination().unwrap(),
        tun.broadcast().unwrap(),
        tun.netmask().unwrap(),
    );

    tun.change_address(Ipv4Addr::new(172, 16, 0, 1))?;
    println!("-----------");
    println!("change address");
    println!("-----------");

    println!(
        "┌ name: {} ({})\n├ fd: {}\n├ mtu: {}\n├ flags: {}\n├ address: {}\n├ destination: {}\n├ broadcast: {}\n└ netmask: {}",
        tun.name(),
        tun.ifidx().unwrap(),
        tun.as_raw_fd(),
        tun.mtu().unwrap(),
        tun.flags().unwrap(),
        tun.address().unwrap(),
        tun.destination().unwrap(),
        tun.broadcast().unwrap(),
        tun.netmask().unwrap(),
    );

    tun.change_destination(Ipv4Addr::new(172, 16, 1, 1))?;
    println!("-----------");
    println!("change destination");
    println!("-----------");

    println!(
        "┌ name: {} ({})\n├ fd: {}\n├ mtu: {}\n├ flags: {}\n├ address: {}\n├ destination: {}\n├ broadcast: {}\n└ netmask: {}",
        tun.name(),
        tun.ifidx().unwrap(),
        tun.as_raw_fd(),
        tun.mtu().unwrap(),
        tun.flags().unwrap(),
        tun.address().unwrap(),
        tun.destination().unwrap(),
        tun.broadcast().unwrap(),
        tun.netmask().unwrap(),
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    Ok(())
}
