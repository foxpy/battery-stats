use getopts::Options;
use std::fs::File;
use std::{env, fmt, process};

#[derive(PartialEq, Eq, Hash)]
struct Decimal {
    val: u64,
    fract: u8,
}

impl Decimal {
    fn new(x: f64) -> Decimal {
        let val = x as u64;
        let fract = ((x * 100.0) as u64 % 100) as u8;
        Decimal{val, fract}
    }
}

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
    fn new(input: &Vec<f64>) -> StatisticalPopulation {
        let mut sp = StatisticalPopulation::default();
        sp.statistical_population = input.clone();
        sp.variation_series = input.clone();
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

    fn plot_frequency_range(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn plot_histogram(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
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
    let brief = format!("Usage: {} [options] input.csv", program_name);
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

fn sample(input: &Vec<f64>, start: usize, n: usize) -> Vec<f64> {
    input
        .clone()
        .into_iter()
        .skip(start-1)
        .enumerate()
        .filter(|(i, _)| i % n == n - 1)
        .map(|(_, v)| v)
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this message");
    opts.optopt("d", "output-directory", "Graph output folder", "DIR");
    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") {
        print_help(&args[0], opts);
    }
    if matches.opt_present("d") {
        env::set_current_dir(matches.opt_str("d").unwrap()).unwrap();
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
