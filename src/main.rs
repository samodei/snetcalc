extern crate clap;
extern crate ipnet;

use std::net::IpAddr;
use clap::{Arg, App, SubCommand, value_t};
use ipnet::IpNet;

/* TODO
 * Give a 10.0.0.0/16
 * print broadcast address
 * print max hosts
 * convert to decimal mast
 * */

fn get_hosts(net: IpNet) {
    // TODO make your own.
}

fn get_broadcast(net: IpNet) -> IpAddr {
    // TODO make your own.
    return net.broadcast();
}

fn to_decimal(net: IpNet) -> IpAddr {
    // TODO make your own.
    return net.netmask();
}

fn subnet(cidr: u32) { // This is old.
    let subnet: u32 = 32 - cidr;
    let hosts = u32::pow(2, subnet) - 2;
    println!("There are {} hosts on a /{} subnet.", hosts, cidr);
}

fn main() {
    let matches = App::new("snetcalc")
        .version("0.0")
        .author("Stefano Amodei <stefano.amodei@pm.me>")
        .about("Calculates subnets.")
        //.license("BSD 3-Clause")
        .arg(Arg::with_name("cidr")
             .short("c")
             .long("cidr")
             .value_name("CIDR"))
        .get_matches();

    if matches.is_present("cidr") {
        let cidr = value_t!(matches.value_of("cidr"), String).unwrap_or_else(|e| e.exit());
        let net: IpNet = cidr.parse().unwrap();
        // println!("{}", net);

        // move this somewhere else
        println!("{:?}", get_broadcast(net));
        println!("{:?}", to_decimal(net));
    } else {
        println!("no");
    }
}
