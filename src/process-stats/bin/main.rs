use getopts::Options;
use itertools::Itertools;
use plotters::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::{env, fmt, mem, process};

#[derive(Debug)]
struct HashedDouble(f64);

impl HashedDouble {
    fn key(&self) -> u64 {
        unsafe { mem::transmute(self.0) }
    }
}

impl Hash for HashedDouble {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

impl PartialEq for HashedDouble {
    fn eq(&self, other: &HashedDouble) -> bool {
        self.key() == other.key()
    }
}

impl Eq for HashedDouble {}

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
        let root = BitMapBackend::new(path, (1280, 720)).into_drawing_area();
        root.fill(&WHITE)?;
        let min = *self
            .variation_series
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max = *self
            .variation_series
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let mut repetitions = HashMap::<HashedDouble, u64>::new();
        for val in self.variation_series.iter() {
            let entry = repetitions.entry(HashedDouble(*val)).or_insert(0);
            *entry += 1;
        }
        let upper = *repetitions
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .1
            + 1;
        let mut chart = ChartBuilder::on(&root)
            .caption("Полигон частот", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_ranged(min..max, 0f64..(upper as f64))?;
        chart.configure_mesh().draw()?;
        chart.draw_series(LineSeries::new(
            repetitions
                .iter()
                .map(|(a, b)| (a.0, *b as f64))
                .sorted_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap()),
            &RED,
        ))?;
        chart.configure_series_labels().draw()?;
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
        .skip(start - 1)
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
    general
        .plot_frequency_range("general_frequency_range.png")
        .unwrap();
    each_second
        .plot_frequency_range("each_second_frequency_range.png")
        .unwrap();
    each_fifth
        .plot_frequency_range("each_fifth_frequency_range.png")
        .unwrap();
    each_fifth_from_second
        .plot_frequency_range("each_fifth_from_second_frequency_range.png")
        .unwrap();
}
