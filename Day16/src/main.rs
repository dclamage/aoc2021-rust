use bitvec::prelude::*;
use std::fs;
use std::time;

type BS = BitSlice<Msb0, u8>;

fn part1(packet_data: &BS) -> usize {
    let mut reader = DataReader::new(packet_data);
    let packet = Packet::new(&mut reader);
    packet.total_version()
}

fn part2(packet_data: &BS) -> usize {
    let mut reader = DataReader::new(packet_data);
    let packet = Packet::new(&mut reader);
    packet.calc_value()
}

#[derive(Debug, Clone)]
struct Packet {
    v: u8,
    t: u8,
    literal: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new(reader: &mut DataReader) -> Packet {
        let v = reader.read_data_u8(3);
        let t = reader.read_data_u8(3);
        let literal = if t == 4 {
            let mut literal: usize = 0;
            loop {
                let has_more = reader.read_data_u8(1);
                let segment = reader.read_data_usize(4);
                literal = (literal << 4) | segment;

                if has_more == 0 {
                    break;
                }
            }

            literal
        } else {
            0usize
        };

        let mut sub_packets = Vec::new();
        if t != 4 {
            let length_type_id = reader.read_data_u8(1);
            if length_type_id == 0 {
                // 15 bit length - number of bits used by sub-packets
                let length = reader.read_data_usize(15);
                let starting_offset = reader.offset;
                while reader.offset - starting_offset < length {
                    let packet = Packet::new(reader);
                    sub_packets.push(packet);
                }
            } else {
                // 11 bit length - number of sub-packets
                let length = reader.read_data_usize(11);
                for _ in 0..length {
                    let packet = Packet::new(reader);
                    sub_packets.push(packet);
                }
            }
        }

        Packet {
            v,
            t,
            literal,
            sub_packets,
        }
    }

    fn total_version(&self) -> usize {
        self.v as usize + self.sub_packets.iter().map(|p| p.total_version()).sum::<usize>()
    }

    fn calc_value(&self) -> usize {
        match self.t {
            0 => self.sub_packets.iter().map(|p| p.calc_value()).sum::<usize>(),
            1 => self.sub_packets.iter().map(|p| p.calc_value()).product::<usize>(),
            2 => self.sub_packets.iter().map(|p| p.calc_value()).min().unwrap_or(0),
            3 => self.sub_packets.iter().map(|p| p.calc_value()).max().unwrap_or(0),
            4 => self.literal,
            5 => if self.sub_packets[0].calc_value() > self.sub_packets[1].calc_value() { 1 } else { 0 },
            6 => if self.sub_packets[0].calc_value() < self.sub_packets[1].calc_value() { 1 } else { 0 },
            7 => if self.sub_packets[0].calc_value() == self.sub_packets[1].calc_value() { 1 } else { 0 },
            _ => panic!("Unknown type"),
        }
    }
}

struct DataReader<'a> {
    data: &'a BS,
    offset: usize,
}

impl DataReader<'_> {
    fn new(data: &BS) -> DataReader {
        DataReader { data, offset: 0 }
    }

    fn read_data_u8(&mut self, size: usize) -> u8 {
        let result = to_int_u8(self.data.get(self.offset..self.offset + size).unwrap());
        self.offset += size;
        result
    }

    fn read_data_usize(&mut self, size: usize) -> usize {
        let result = to_int_usize(self.data.get(self.offset..self.offset + size).unwrap());
        self.offset += size;
        result
    }
}

fn to_int_u8(bits: &BS) -> u8 {
    bits.iter().fold(0, |acc, x| acc * 2 + *x as u8)
}

fn to_int_usize(bits: &BS) -> usize {
    bits.iter().fold(0, |acc, x| acc * 2 + *x as usize)
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example3.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<u8> = hex::decode(input.trim()).expect("Unable to decode hex");
    let input_bits = input.view_bits::<Msb0>();

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&input_bits);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&input_bits);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
