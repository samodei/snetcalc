extern crate clap;

use clap::{Arg, App, SubCommand, value_t};
/*
fn get_broadcast(cidr: Ipv4Net) {
    println!("{:?}", cidr);
}

fn to_decimal() {
    println!("decimal");
}
*/
/*fn get_range(cidrnet: Ipv4Net) {
    println!("{:?}", cidrnet);
}*/

fn get_maxhosts(maskbits: u32) -> u32 {
    // TODO Rewrite to take output from parse_addr
    // Return total number of hosts.
    return u32::pow(2, 32 - maskbits);
}

fn get_usablehosts(maxhosts: u32) -> u32 {
    // TODO Rewrite to take output from parse_addr
    // Return total number of usable hosts.
    return maxhosts - 2;
}

fn get_wildcard(input: Vec<u8>) -> Vec<u8> {
    // Return wildcard mask.
    let mut wildcard: Vec<u8> = vec![];
    for octet in input {
        wildcard.push(255 - octet);
    }

    return wildcard;

}

fn parse_addr(input: String) -> Vec<u8> {
    // Parse the input and store each octet in an array as u8

    // Split the address by octet.
    let addr: Vec<&str> = input.split(".").collect();
    let mut addrint: Vec<u8> = vec![];

    for octet in addr {
        // String to u8 and push to addrint.
        addrint.push(octet.parse().unwrap());
    }

    return addrint;
}

fn to_binary(input: Vec<u8>) -> Vec<u32> {
    // Parse the ouput of parse_addr to convert to binary. 
    let mut addrbin: Vec<u32> = vec![]; // getting an error when using u8
    for octet in input {
        addrbin.push(format!("{:b}", octet).parse().unwrap());
    }

    return addrbin;

}

fn get_slash(input: Vec<u32>) -> u32 {
    // Take in binary and add the ones to get slash.
    let mut convert: Vec<u32> = vec![];
    for octet in &input {
        let addrstr: String = octet.to_string();
        let char_vec: Vec<char> = addrstr.chars().collect();
        for c in char_vec {
            let test: String = c.to_string();
            let test2: u32 = test.parse().unwrap();
            convert.push(test2);
        }
        
    }

    let slash: u32 = convert.iter().sum();
    return slash;
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
        .arg(Arg::with_name("wildcard")
              .short("w")
              .long("wildcard")
              .value_name("wildcard"))
        .get_matches();

    if matches.is_present("cidr") {
        // NEW TEST
        // Take string then make to ipv4net
        let cidr = value_t!(matches.value_of("cidr"), String).unwrap_or_else(|e| e.exit());

        //test_parse(parse_addr(cidr));
        to_binary(parse_addr(cidr));

        //let cidrnet: Ipv4Net = cidr.parse().unwrap();
        //assert_eq!(Ok(cidrnet.network()), cidr.parse());
        //println!("{:?}", cidrnet);
    } else if matches.is_present("hosts") {
        let hosts = value_t!(matches.value_of("hosts"), u32).unwrap_or_else(|e| e.exit());
        println!("Total number of hosts: {:?}", get_maxhosts(hosts));
        println!("Total number of usable hosts: {:?}", get_usablehosts(get_maxhosts(hosts)));
    } else if matches.is_present("subnet") {
        // Convert from subnet mask to slash notation.
        let input = value_t!(matches.value_of("subnet"), String).unwrap_or_else(|e| e.exit());

        let binary = to_binary(parse_addr(input));
        println!("{:?}", get_slash(binary));
    } else if matches.is_present("wildcard") {
        let input = value_t!(matches.value_of("wildcard"), String).unwrap_or_else(|e| e.exit());
        // fix this formatting
        println!("{:?}", get_wildcard(parse_addr(input)));
    } else {
        println!("no");
    }
}
