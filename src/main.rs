use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct RefData {
    pub pilots: Vec<String>,
    pub players: Vec<String>,
    pub maps: Vec<String>,
}

fn main() {
    let file = File::open("config.json").unwrap();
    let buf_reader = BufReader::new(file);

    let reference: RefData = serde_json::from_reader(buf_reader).unwrap();

    // Randomizes wich pilot will be used by each player. Pilot cannot be repeated between players
    let mut used: Vec<usize> = Vec::new();
    let mut rng = thread_rng();

    let mut player_pilot: Vec<(String, String)> = Vec::new();

    for player in reference.players {
        let mut number: usize;

        // Deve retentar enquanto o numero gerado pelo RNG existir nos números utilizados
        loop {
            number = rng.gen_range(1..=reference.pilots.len());
            if !used.contains(&number) {
                used.push(number);
                break;
            }
        }

        player_pilot.push((player, reference.pilots[number - 1].clone()));
    }

    let pista_id = rng.gen_range(1..=reference.maps.len());
    let pista = reference.maps[pista_id - 1].clone();

    // Prepara um arquivo com a composição
    let out_file = File::create("f1-settings").unwrap();
    let mut buf_writer = BufWriter::new(out_file);

    buf_writer
        .write_fmt(format_args!("F1 RANDOMIZER - by Henrique Starosky\n\n"))
        .unwrap();

    buf_writer
        .write_fmt(format_args!("Track selected: {0}\n", pista,))
        .unwrap();

    buf_writer
        .write_fmt(format_args!("Players | Drivers\n"))
        .unwrap();

    for (player, pilot) in player_pilot {
        // Prepara os espaços para ficar bonito
        buf_writer
            .write_fmt(format_args!("{0}: {1}\n", player, pilot))
            .unwrap();
    }
}
