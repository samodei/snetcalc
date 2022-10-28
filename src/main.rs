extern crate clap;

use clap::{Arg, App, value_t};

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

fn slash_to_binary(input: u32) -> Vec<u8> {
    // From slash notation to binary.
    let mut slashbin: Vec<u8> = vec![];
    slashbin.push(format!("{:b}", input).parse().unwrap());

    return slashbin;
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
        .version("0.1")
        .author("Stefano Amodei <stefano.amodei@pm.me>")
        .about("Calculates subnets.")
        .arg_required_else_help(true)
       // .license("BSD 3-Clause")
        .arg(Arg::with_name("cidr")
             .short('c')
             .long("cidr")
             .value_name("CIDR"))
        .arg(Arg::with_name("subnet")
             .short('s')
             .long("subnet")
             .value_name("subnet"))
        .arg(Arg::with_name("wildcard")
              .short('w')
              .long("wildcard")
              .value_name("wildcard"))
        .get_matches();

    if matches.is_present("cidr") {
        let input = value_t!(matches.value_of("cidr"), String).unwrap_or_else(|e| e.exit());

        let slash: u32 = input.parse().unwrap();
        // Get hosts using CIDR notation.
        let max_hosts = get_maxhosts(slash);
        let usable_hosts = get_usablehosts(max_hosts);

        let test = slash_to_binary(slash);
        println!("{:?}", test);
        println!("CIDR Notation: /{:?}", input);
        println!("Total number of hosts: {:?}", max_hosts);
        println!("Total number of usable hosts: {:?}", usable_hosts);
    } else if matches.is_present("subnet") {
        // Convert from subnet mask to slash notation.
        let input = value_t!(matches.value_of("subnet"), String).unwrap_or_else(|e| e.exit());

        // Parse input and convert from decimal subnet mask to binary.
        let binary = to_binary(parse_addr(input));

        // Convert binary subnet mask to CIDR notation.
        let slash = get_slash(binary);

        // Get hosts using CIDR notation.
        let max_hosts = get_maxhosts(slash);
        let usable_hosts = get_usablehosts(max_hosts);

        println!("CIDR Notation: /{:?}", slash);
        //println!("Wildcard Mask: {:?}", wildcard);
        println!("Total number of hosts: {:?}", max_hosts);
        println!("Total number of usable hosts: {:?}", usable_hosts);
    } else if matches.is_present("wildcard") {
        let input = value_t!(matches.value_of("wildcard"), String).unwrap_or_else(|e| e.exit());
        // fix this formatting
        println!("{:?}", get_wildcard(parse_addr(input)));
    } else {
        println!("no");
    }
}
