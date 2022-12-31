use std::io::Result;

use read_input::read_text;

fn get_binary_from_hex(hex: char) -> String {
    match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => {
            panic!("Unrecognized hex character {}", hex)
        }
    }
    .to_owned()
}

fn get_number_from_range(bin: &Vec<u8>, start: usize, end: usize) -> usize {
    if bin.len() <= end {
        panic!("Invalid string {:?} for range {} -> {}", bin, start, end);
    }
    let slice = &bin[start..=end];
    slice
        .iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, b)| *b as usize * 2usize.pow(i as u32) + sum)
}

fn get_vec_from_slice(slice: &[u8]) -> Vec<u8> {
    let mut vec = Vec::with_capacity(slice.len());

    for n in slice {
        vec.push(*n);
    }

    vec
}

struct ParsedPacket {
    version_sum: usize,
    literal_values: Vec<usize>,
    resulting_length: Option<usize>,
}

impl ParsedPacket {
    fn new(version: usize) -> Self {
        ParsedPacket {
            version_sum: version,
            literal_values: Vec::new(),
            resulting_length: None,
        }
    }

    fn append(&mut self, mut other: ParsedPacket) {
        self.version_sum += other.version_sum;
        self.literal_values.append(&mut other.literal_values);
        if other.resulting_length.is_some() {
            if self.resulting_length.is_some() {
                self.add_to_length(other.resulting_length.unwrap());
            } else {
                self.resulting_length = other.resulting_length.clone();
            }
        }
    }

    fn add_to_length(&mut self, amount: usize) {
        self.resulting_length = Some(self.resulting_length.unwrap() + amount);
    }

    fn process_op_code(&mut self, code: usize) {
        match code {
            0 => {
                self.literal_values =
                    vec![self.literal_values.iter().fold(0, |sum, value| sum + value)];
            }
            1 => {
                self.literal_values =
                    vec![self.literal_values.iter().fold(1, |sum, value| sum * value)];
            }
            2 => {
                let min = self.literal_values.iter().min().unwrap();
                self.literal_values = vec![*min];
            }
            3 => {
                let max = self.literal_values.iter().max().unwrap();
                self.literal_values = vec![*max];
            }
            5 => {
                if self.literal_values.len() != 2 {
                    println!(
                        "had more than two values in sub packet for greater than op {:?}",
                        self.literal_values
                    );
                }
                let value = if self.literal_values[0] > self.literal_values[1] {
                    1
                } else {
                    0
                };
                self.literal_values = vec![value];
            }
            6 => {
                if self.literal_values.len() != 2 {
                    println!(
                        "had more than two values in sub packet for less than op {:?}",
                        self.literal_values
                    );
                }
                let value = if self.literal_values[0] < self.literal_values[1] {
                    1
                } else {
                    0
                };
                self.literal_values = vec![value];
            }
            7 => {
                if self.literal_values.len() != 2 {
                    println!(
                        "had more than two values in sub packet for equal to op {:?}",
                        self.literal_values
                    );
                }
                let value = if self.literal_values[0] == self.literal_values[1] {
                    1
                } else {
                    0
                };
                self.literal_values = vec![value];
            }
            _ => panic!("Unrecognized op code: {}", code),
        }
    }
}

fn parse_packet(binary: &Vec<u8>) -> ParsedPacket {
    let version = get_number_from_range(binary, 0, 2);
    let type_id = get_number_from_range(binary, 3, 5);
    let mut parsed_packet = ParsedPacket::new(version);

    // println!("version: {}, type_id: {}", version, type_id);
    // println!("{:?}", binary);

    match type_id {
        // contains a single number broken into segments of 5
        4 => {
            let sub_number_starting_point = 6;
            let mut section_start = sub_number_starting_point;
            let mut numbers = Vec::new();
            let mut number_of_digits_scanned = 0;
            loop {
                for n in &binary[section_start + 1..=section_start + 4] {
                    numbers.push(*n);
                }
                number_of_digits_scanned += 5;
                if binary[section_start] == 0 {
                    break;
                }
                section_start += 5;
            }

            // the length should include the version, type id, and the groups of 5 making up the number
            parsed_packet.resulting_length =
                Some(sub_number_starting_point + number_of_digits_scanned);

            parsed_packet.literal_values.push(get_number_from_range(
                &numbers,
                0,
                numbers.len() - 1,
            ));
        }
        _ => {
            let length_type_id = get_number_from_range(binary, 6, 6);
            match length_type_id {
                0 => {
                    // start at 7, after version, type id, and length
                    let start = 7;
                    // get the next 15 bits, inclusive
                    let end = start + 14;
                    let total_sub_packet_length = get_number_from_range(binary, start, end);
                    // have the first packet start right after the length info.
                    let mut sub_packet_start = end + 1;
                    let mut sub_packet_length_sum = 0;
                    loop {
                        // get a vec of the binary from the starting point to the end of the whole thing
                        // (we dont know where it ends yet)
                        let sub_packet_binary =
                            get_vec_from_slice(&binary[sub_packet_start..binary.len()]);
                        // parse the sub packet, letting it determine version and handle the literal value
                        let sub_parsed_packet = parse_packet(&sub_packet_binary);
                        if sub_parsed_packet.resulting_length.is_none() {
                            panic!(
                                "Sub packet did not return its length {:?}",
                                sub_packet_binary
                            );
                        }
                        let length = sub_parsed_packet.resulting_length.unwrap();
                        parsed_packet.append(sub_parsed_packet);
                        // increment the start for the next subpacket but the digits just scanned
                        sub_packet_start += length;
                        // increase the total digit use for the subpackets
                        sub_packet_length_sum += length;
                        // exit if we passed the total length
                        if sub_packet_length_sum >= total_sub_packet_length {
                            break;
                        }
                    }
                    // add 1 here to move the cursor to the next digit, dont count the ending one twice
                    parsed_packet.add_to_length(end + 1);
                }
                1 => {
                    // start at 7, after version, type id, and length
                    let start = 7;
                    // get the next 11 bits, inclusive
                    let end = start + 10;
                    let number_of_sub_packets = get_number_from_range(binary, start, end);
                    let mut sub_packet_start = end + 1;
                    for _ in 0..number_of_sub_packets {
                        let sub_packet_binary =
                            get_vec_from_slice(&binary[sub_packet_start..binary.len()]);
                        // parse the sub packet, letting it determine version and handle the literal value
                        let sub_parsed_packet = parse_packet(&sub_packet_binary);
                        if sub_parsed_packet.resulting_length.is_none() {
                            panic!(
                                "Sub packet did not return its length {:?}",
                                sub_packet_binary
                            );
                        }

                        // increment the start for the next subpacket but the digits just scanned
                        sub_packet_start += sub_parsed_packet.resulting_length.unwrap();

                        parsed_packet.append(sub_parsed_packet);
                    }
                    // add 1 here to move the cursor to the next digit, dont count the ending one twice
                    parsed_packet.add_to_length(end + 1);
                }
                _ => panic!("Invalid length type id {} for {:?}", length_type_id, binary),
            }
            parsed_packet.process_op_code(type_id);
        }
    }

    parsed_packet
}

fn main() -> Result<()> {
    let text = read_text("16/input.txt")?;

    for line in text.lines() {
        println!("{}", line);
        let binary =
            line.chars()
                .map(get_binary_from_hex)
                .fold(Vec::new(), |mut list, binary_digits| {
                    list.append(
                        &mut binary_digits
                            .chars()
                            .map(|d| {
                                if let Some(d) = d.to_digit(10) {
                                    d as u8
                                } else {
                                    panic!("Could not convert digit to base 10: {}", d);
                                }
                            })
                            .collect::<Vec<u8>>(),
                    );
                    list
                });

        let parsed_packet = parse_packet(&binary);
        println!(
            "version sum: {}\nvalues: {:?}\n",
            parsed_packet.version_sum, parsed_packet.literal_values
        );
    }

    Ok(())
}
