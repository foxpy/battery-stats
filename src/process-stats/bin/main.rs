use getopts::Options;
use std::fs::File;
use std::{env, process};

fn print_help(program_name: &str, opts: Options) -> ! {
    let brief = format!("Usage: {} input.csv", program_name);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn read_input_file(file: &str) -> std::io::Result<Vec<f64>> {
    let mut measures = vec![];
    let file = File::open(file)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    for i in reader.records() {
        let record = i?;
        let power: f64 = record[1].parse().unwrap();
        measures.push(power);
    }
    Ok(measures)
}

fn process_data(input: &Vec<f64>) {
    let mut sorted = input.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let average = input.iter().sum::<f64>() / input.len() as f64;
    let dispersion =
        (input.iter().map(|x| x.powf(2.0)).sum::<f64>() / input.len() as f64) - average.powf(2.0);
    let stddev = dispersion.sqrt();
    println!("Генеральная совокупность: {:?}", input);
    println!("Вариационный ряд: {:?}", sorted);
    println!("Среднее значение: {:.5}", average);
    println!("Дисперсия: {:.5}", dispersion);
    println!("Стандартное отклонение: {:.5}", stddev);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this message");
    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") {
        print_help(&args[0], opts);
    }
    if matches.free.is_empty() {
        panic!("No input file specified");
    }

    let measures = read_input_file(&matches.free[0]).unwrap();
    process_data(&measures);
    println!("\nВыборка [каждый второй]:");
    let each_second = measures
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    process_data(&each_second);
    println!("\nВыборка [каждый пятый]:");
    let each_fifth = measures
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 5 == 1)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    process_data(&each_fifth);
}
