use std::env;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::process;

#[derive(Debug)]
enum EncodingField {
    X,
    Y,
    X2pY2,
    X2mY2,
    X2Y2,
}

impl EncodingField {
    pub fn from_name(name: &str) -> Result<Self, Box<dyn Error>> {
        match name.to_lowercase().as_str() {
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "x2py2" => Ok(Self::X2pY2),
            "x2my2" => Ok(Self::X2mY2),
            "x2y2" => Ok(Self::X2Y2),
            other => Err(format!("Unknown field name: {}", other).into()),
        }
    }
    pub fn derivative_at(&self, pos: &[f64; 2]) -> [f64; 2] {
        match *self {
            Self::X => [1.0, 0.0],
            Self::Y => [0.0, 1.0],
            Self::X2pY2 => [pos[0], pos[1]],
            Self::X2mY2 => [2.0 * pos[0], -2.0 * pos[1]],
            Self::X2Y2 => [2.0 * pos[1], 2.0 * pos[0]],
        }
    }
}

fn localkspace(sample: &[f64], derivatives: &[[f64; 2]]) -> [f64; 2] {
    [
        derivatives
            .iter()
            .zip(sample.iter())
            .map(|(di, si)| di[0] * si)
            .sum(),
        derivatives
            .iter()
            .zip(sample.iter())
            .map(|(di, si)| di[1] * si)
            .sum(),
    ]
}

fn run() -> Result<(), Box<dyn Error>> {
    // Get position local k space should be evaluated at
    let pos = parse_position()?;

    // Acquire stdin/stdout
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Read in k space sampling positions
    let mut header = String::new();

    // Get and parse SEM field shape identifiers and obtain the derivative at `pos`
    stdin.read_line(&mut header)?;
    let d: Vec<_> = header
        .trim()
        .split(',')
        .map(|x| EncodingField::from_name(x).unwrap().derivative_at(&pos))
        .collect();

    // Lock stdout and write header
    let mut handle = stdout.lock();
    writeln!(handle, "k1,k2")?;

    // Read input line by line, parse, compute local k space and print
    for sample in stdin.lock().lines() {
        // Parsing
        let s: Vec<f64> = sample?
            .split(',')
            .map(|x| x.parse::<f64>().expect("Corrupted input."))
            .collect();
        // Computing local kspace
        let k = localkspace(&s, &d);
        // Printing
        writeln!(handle, "{},{}", k[0], k[1])?;
    }
    Ok(())
}

fn parse_position() -> Result<[f64; 2], Box<dyn Error>> {
    let o: Vec<_> = env::args_os()
        // First one is the name of the binary
        .skip(1)
        // Only interested in two arguments, ignore the rest
        .take(2)
        .map(|x| {
            // Convert to string
            x.into_string()
                .expect("Argument not a valid string")
                // Parse as f64
                .parse::<f64>()
                .expect("Argument does not seem to be a number")
        })
        .collect();
    Ok([o[0], o[1]])
}

fn main() {
    if let Err(err) = run() {
        println!("Error computing local k space: {}", err);
        process::exit(1);
    }
}
