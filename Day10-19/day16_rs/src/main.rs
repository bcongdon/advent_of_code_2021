use bitvec::prelude::*;
use std::{num::ParseIntError, u128};

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn apply(&self, values: &[u128]) -> u128 {
        let bool_to_value = |pred| if pred { 1 } else { 0 };
        match self {
            &Operator::Sum => values.iter().sum(),
            &Operator::Product => values.iter().product(),
            &Operator::Maximum => values.iter().max().cloned().unwrap(),
            &Operator::Minimum => values.iter().min().cloned().unwrap(),
            &Operator::LessThan => bool_to_value(values[0] < values[1]),
            &Operator::GreaterThan => bool_to_value(values[0] > values[1]),
            &Operator::EqualTo => bool_to_value(values[0] == values[1]),
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(u128),
    Operator(Operator),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    typ: PacketType,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.subpackets.iter().map(|p| p.version_sum()).sum::<u32>() + self.version as u32
    }

    fn eval(&self) -> u128 {
        match &self.typ {
            PacketType::Literal(val) => *val,
            PacketType::Operator(op) => {
                let subpacket_values = self.subpackets.iter().map(Packet::eval).collect::<Vec<_>>();
                op.apply(&subpacket_values)
            }
        }
    }
}

fn parse_packet(bits: &BitSlice<Msb0, u8>) -> (Packet, usize) {
    let version = bits[0..3].load_be::<u8>();
    let type_id = bits[3..6].load_be::<u8>();

    let mut terminal_idx = 6;

    let (packet_type, subpackets) = match type_id {
        4 => {
            let mut literal = BitVec::<Msb0, u8>::new();
            loop {
                let chunk = &bits[terminal_idx..terminal_idx + 5];
                terminal_idx += 5;
                let done = !chunk[0];
                literal.append(&mut chunk[1..].to_bitvec());
                if done {
                    break;
                }
            }
            (PacketType::Literal(literal.load_be()), vec![])
        }
        _ => {
            let length_type_id = bits[terminal_idx];
            terminal_idx += 1;

            let mut subpackets = Vec::new();
            if length_type_id {
                let num_sub_packets = bits[terminal_idx..terminal_idx + 11].load_be::<u16>();
                terminal_idx += 11;
                for _ in 0..num_sub_packets {
                    let (subpacket, offset) = parse_packet(&bits[terminal_idx..]);
                    terminal_idx += offset;
                    subpackets.push(subpacket);
                }
            } else {
                let total_length = bits[terminal_idx..terminal_idx + 15].load_be::<usize>();
                terminal_idx += 15;
                let subpacket_start_idx = terminal_idx;
                while terminal_idx < subpacket_start_idx + total_length {
                    let (subpacket, offset) =
                        parse_packet(&bits[terminal_idx..subpacket_start_idx + total_length]);
                    terminal_idx += offset;
                    subpackets.push(subpacket);
                }
            }
            let operator = match type_id {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Minimum,
                3 => Operator::Maximum,
                5 => Operator::GreaterThan,
                6 => Operator::LessThan,
                7 => Operator::EqualTo,
                _ => panic!("Invalid operator"),
            };
            (PacketType::Operator(operator), subpackets)
        }
    };

    (
        Packet {
            subpackets,
            version,
            typ: packet_type,
        },
        terminal_idx,
    )
}

fn main() {
    let input = include_str!("input.txt").trim();
    let hex_bits = decode_hex(input).unwrap();

    let hex_bits = hex_bits.view_bits::<Msb0>();
    let (packet, _) = parse_packet(hex_bits);
    println!("Part 1: {}", packet.version_sum());
    println!("Part 2: {}", packet.eval());
}
