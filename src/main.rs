use clap::{Arg, Command, Parser, Subcommand};
use header_accumulator::errors::EraValidateError;
use std::{io::BufReader, process};
use trin_validation::accumulator::PreMergeAccumulator;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Stream data continuously
    Stream {
        #[clap(short, long, default_value = "false")]
        decompress: bool,
        #[clap(short, long)]
        end_block: Option<usize>,
    },
    /// Decode files from input to output
    Decode {
        #[clap(short, long)]
        input: String,
        #[clap(long)]
        headers_dir: Option<String>,
    },
}

fn main() {
    env_logger::init();
    let matches = Command::new("header_accumulator")
        .version("0")
        .author("Semiotic Labs")
        .about("Validates flat files against Header Accumulators")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("era_validate")
                .about("Validates entire ERAs of flat files against Header Accumulators")
                .arg(
                    Arg::new("directory")
                        .help("Directory where the flat files are stored")
                        .required(false)
                        .index(1),
                )
                .arg(
                    Arg::new("start_epoch")
                        .help("Start epoch to check")
                        .required(false)
                        .short('s')
                        .long("start_epoch"),
                )
                .arg(
                    Arg::new("end_epoch")
                        .help("End epoch to check")
                        .required(false)
                        .short('e')
                        .long("end_epoch"),
                )
                .arg(
                    Arg::new("pre_merge_accumulator_file")
                        .help("pre-merge accumulator file (optional)")
                        .required(false)
                        .short('m')
                        .long("pre_merge_accumulator_file"),
                )
                .subcommand(
                    Command::new("stream")
                        .about("Validates streams ERAs of flat files against Header Accumulators")
                        .arg(
                            Arg::new("pre_merge_accumulator_file")
                                .help("pre-merge accumulator file (optional)")
                                .required(false)
                                .short('m')
                                .long("pre_merge_accumulator_file"),
                        ),
                ),
        )
        .subcommand(
            Command::new("generate_inclusion_proof")
                .about("Generates inclusion proofs for a range of blocks")
                .arg(
                    Arg::new("directory")
                        .help("Directory where the flat files are stored")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("start_block")
                        .help("Start block to generate inclusion proof for")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("end_block")
                        .help("End block to generate inclusion proof for")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::new("output_file")
                        .help("Output file for the inclusion proof")
                        .required(false)
                        .short('o')
                        .long("output_file"),
                ),
        )
        .subcommand(
            Command::new("verify_inclusion_proof")
                .about("Verifies inclusion proofs for a range of blocks")
                .arg(
                    Arg::new("directory")
                        .help("Directory where the flat files are stored")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("start_block")
                        .help("Start block to verify inclusion proof for")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("end_block")
                        .help("End block to verify inclusion proof for")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::new("inclusion_proof_file")
                        .help("Inclusion proof to verify")
                        .required(true)
                        .index(4),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        // TODO: move this functionality to flat_head
        Some(("era_validate", era_validate_matches)) => {
            if let Some(("stream", stream_matches)) = era_validate_matches.subcommand() {
                let pre_merge_accumulator_file =
                    stream_matches.get_one::<String>("pre_merge_accumulator_file");
                let _pre_merge_accumulator = match pre_merge_accumulator_file {
                    Some(pre_merge_accumulator_file) => {
                        PreMergeAccumulator::try_from_file(pre_merge_accumulator_file.into())
                            .map_err(|_| EraValidateError::InvalidPreMergeAccumulatorFile)
                            .expect("Invalid pre-merge accumulator file")
                    }
                    None => PreMergeAccumulator::default(),
                };
                let _reader = BufReader::with_capacity(1 << 32, std::io::stdin().lock());
                let _writer = std::io::stdout();
                process::exit(0);
            }
        }
        //TODO: move this functionality to flat_head
        // Some(("generate_inclusion_proof", generate_inclusion_proof_matches)) => {
        //     let directory = generate_inclusion_proof_matches
        //         .get_one::<String>("directory")
        //         .expect("Directory is required.");
        //     let start_block = generate_inclusion_proof_matches
        //         .get_one::<String>("start_block")
        //         .expect("Start block is required.");
        //     let end_block = generate_inclusion_proof_matches
        //         .get_one::<String>("end_block")
        //         .expect("End block is required.");

        //     let inclusion_proof = inclusion_proof::generate_inclusion_proof(
        //         &directory,
        //         start_block.parse::<usize>().unwrap(),
        //         end_block.parse::<usize>().unwrap(),
        //     )
        //     .expect("Error generating inclusion proof");

        //     let inclusion_proof_serialized = serde_json::to_string(&inclusion_proof).unwrap();
        //     // write the proof to a file
        //     // if output_file is not provided, write to inclusion_proof.json
        //     let output_file = generate_inclusion_proof_matches.get_one::<String>("output_file");
        //     match output_file {
        //         Some(output_file) => {
        //             std::fs::write(output_file.to_owned() + ".json", inclusion_proof_serialized)
        //                 .expect("Unable to write file");
        //         }
        //         None => {
        //             std::fs::write("inclusion_proof.json", inclusion_proof_serialized)
        //                 .expect("Unable to write file");
        //         }
        //     }
        //     process::exit(0);
        // }
        // Some(("verify_inclusion_proof", verify_inclusion_proof_matches)) => {
        //     let directory = verify_inclusion_proof_matches
        //         .get_one::<String>("directory")
        //         .expect("Directory is required.");
        //     let start_block = verify_inclusion_proof_matches
        //         .get_one::<String>("start_block")
        //         .expect("Start block is required.");
        //     let end_block = verify_inclusion_proof_matches
        //         .get_one::<String>("end_block")
        //         .expect("End block is required.");
        //     let inclusion_proof_file = verify_inclusion_proof_matches
        //         .get_one::<String>("inclusion_proof_file")
        //         .expect("Inclusion proof is required.");

        //     // Load inclusion proof
        //     let inclusion_proof = std::fs::read_to_string(inclusion_proof_file)
        //         .expect("Error reading inclusion proof file");
        //     let inclusion_proof: Vec<[H256; 15]> =
        //         serde_json::from_str(&inclusion_proof).expect("Error parsing inclusion proof");

        //     let result = inclusion_proof::verify_inclusion_proof(
        //         &directory,
        //         None,
        //         start_block.parse::<usize>().unwrap(),
        //         end_block.parse::<usize>().unwrap(),
        //         inclusion_proof,
        //     );

        //     if result.is_ok() {
        //         println!("Inclusion proof verified!");
        //         process::exit(0);
        //     } else {
        //         println!("Inclusion proof failed to verify");
        //         process::exit(1);
        //     }
        // }
        _ => {
            println!("No subcommand was used");
        }
    }
}
