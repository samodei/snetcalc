extern crate clap;
extern crate ipnet;

//use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
//use std::net::Ipv4Addr;
use ipnet::Ipv4Net;
use clap::{Arg, App, SubCommand, value_t};
/*
fn get_broadcast(cidr: Ipv4Net) {
    println!("{:?}", cidr);
}

fn to_decimal() {
    println!("decimal");
}
*/
fn get_range(cidrnet: Ipv4Net) {
    println!("{:?}", cidrnet);
}

fn get_maxhosts(maskbits: u32) -> u32 {
    // Return total number of hosts.
    return u32::pow(2, 32 - maskbits);
}

fn get_usablehosts(maxhosts: u32) -> u32 {
    // Return total number of usable hosts.
    return maxhosts - 2;
}

fn get_wildcard(maskbit: u32) -> u32 {
    // Return wildcard mask.
    return 255 - maskbit;
}

fn get_slash() {

}

fn dec_to_bin(input: String) {
    // This may be a very hacky way to do this but the only way I can think of.
    let netmask: Vec<&str> = input.split(".").collect();
    let mut wildcard: Vec<u32> = vec![];

        // I've been trying to understand how to do this for 2 years.
        // Store each bit in an array then sum the array to get the slash notation.
        let mut convert: Vec<u32> = vec![];
        let mut chars: Vec<char> = vec![];

        // Loop by octet.
        for octet in &netmask {
            // String to u32
            let maskdec: u32 = octet.parse().unwrap();

            // Format u32 into binary.
            let maskbit: u32 = format!("{:b}", maskdec).parse().unwrap();
           
            // Make a string then store each character into vector.
            let maskstr: String = maskbit.to_string();
            let char_vec: Vec<char> = maskstr.chars().collect();
            for c in char_vec {
                // Store each char as u32 in vector.
                let test: String = c.to_string();
                let test2: u32 = test.parse().unwrap();
                convert.push(test2);
            }

            // Get the wildcard mask and push to vector.
            wildcard.push(get_wildcard(maskdec));
        }

        // FINALLY SUM THE ARRAY
        let slash: u32 = convert.iter().sum();
        println!("{:?}", slash);

}

fn main() {
    let matches = App::new("snetcalc")
        .version("0.0")
        .author("Stefano Amodei <stefano.amodei@pm.me>")
        .about("Calculates subnets.")
       // .license("BSD 3-Clause")
        .arg(Arg::with_name("cidr")
             .short("c")
             .long("cidr")
             .value_name("CIDR"))
        .arg(Arg::with_name("hosts")
             .short("h")
             .long("hosts")
             .value_name("hosts"))
        .arg(Arg::with_name("subnet")
             .short("s")
             .long("subnet")
             .value_name("subnet"))
        .get_matches();

    if matches.is_present("cidr") {

        // Probably should return all strings to console.
        // Find range of 10.0.0.0/24
        // Parse propper format
        //let cidr = value_t!(matches.value_of("cidr"), String).unwrap_or_else(|e| e.exit());
        //let net: IpNet = cidr.parse().unwrap();
        // println!("{}", net);
        //let net = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0));


        // NEW TEST
        // Take string then make to ipv4net
        let cidr = value_t!(matches.value_of("cidr"), String).unwrap_or_else(|e| e.exit());
        let slash = cidr.split('/');
        println!("{:?}", slash);
        let cidrnet: Ipv4Net = cidr.parse().unwrap();
        //assert_eq!(Ok(cidrnet.network()), cidr.parse());

        let test: u32 = 24;
        println!("{:?}", cidrnet);

    } if matches.is_present("hosts") {
        let hosts = value_t!(matches.value_of("hosts"), u32).unwrap_or_else(|e| e.exit());
        println!("Total number of hosts: {:?}", get_maxhosts(hosts));
        println!("Total number of usable hosts: {:?}", get_usablehosts(get_maxhosts(hosts)));
    } if matches.is_present("subnet") {

        //Going to try from scratch

        let input = value_t!(matches.value_of("subnet"), String).unwrap_or_else(|e| e.exit());
        dec_to_bin(input);
        //println!("Slash: /{:?}", slash);
        //println!("Wildcard mask: {:?}.{:?}.{:?}.{:?}", wildcard[0], wildcard[1], wildcard[2], wildcard[3]);
    } else {
        println!("no");
    }
}
