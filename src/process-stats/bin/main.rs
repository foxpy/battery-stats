use getopts::Options;
use std::fs::File;
use std::{env, fmt, process};

#[derive(Default)]
struct StatisticalPopulation {
    statistical_population: Vec<f64>,
    variation_series: Vec<f64>,
    average: f64,
    dispersion: f64,
    stddev: f64,
    corrected_dispersion: f64,
    corrected_stddev: f64,
}

impl StatisticalPopulation {
    fn new(input: &[f64]) -> StatisticalPopulation {
        let mut sp = StatisticalPopulation::default();
        sp.statistical_population = input.to_owned();
        sp.variation_series = input.to_owned();
        sp.variation_series
            .sort_by(|a, b| a.partial_cmp(b).unwrap());
        sp.average = sp.variation_series.iter().sum::<f64>() / sp.variation_series.len() as f64;
        sp.dispersion = sp
            .variation_series
            .iter()
            .map(|x| (x - sp.average).powf(2.0))
            .sum::<f64>()
            / sp.variation_series.len() as f64;
        sp.stddev = sp.dispersion.sqrt();
        sp.corrected_dispersion = sp.dispersion * sp.variation_series.len() as f64
            / (sp.variation_series.len() - 1) as f64;
        sp.corrected_stddev = sp.corrected_dispersion.sqrt();
        sp
    }
}

impl fmt::Display for StatisticalPopulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Генеральная совокупность: {:?}",
            self.statistical_population
        )?;
        writeln!(f, "Вариационный ряд: {:?}", self.variation_series)?;
        writeln!(f, "Математическое ожидание: {:.5}", self.average)?;
        writeln!(f, "Дисперсия: {:.5}", self.dispersion)?;
        writeln!(f, "Стандартное отклонение: {:.5}", self.stddev)?;
        writeln!(
            f,
            "Дисперсия исправленная: {:.5}",
            self.corrected_dispersion
        )?;
        write!(
            f,
            "Стандартное отклонение исправленное: {:.5}",
            self.corrected_stddev
        )
    }
}

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

fn sample(input: &[f64], start: usize, n: usize) -> Vec<f64> {
    input
        .iter()
        .skip(start-1)
        .enumerate()
        .filter(|(i, _)| i % n == n - 1)
        .map(|(_, v)| *v)
        .collect()
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
    let general = StatisticalPopulation::new(&measures);
    let each_second = StatisticalPopulation::new(&sample(&measures, 1, 2));
    let each_fifth = StatisticalPopulation::new(&sample(&measures, 1, 5));
    let each_fifth_from_second = StatisticalPopulation::new(&sample(&measures, 2, 5));
    println!("{}", general);
    println!("\nВыборка [каждый второй]:\n{}", each_second);
    println!("\nВыборка [каждый пятый]:\n{}", each_fifth);
    println!(
        "\nВыборка [каждый пятый со второго]:\n{}",
        each_fifth_from_second
    );
}
