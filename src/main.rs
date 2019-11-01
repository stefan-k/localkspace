use std::env;
use std::error::Error;
use std::io;
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
        match name {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "X2pY2" => Ok(Self::X2pY2),
            "X2mY2" => Ok(Self::X2mY2),
            "X2Y2" => Ok(Self::X2Y2),
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

fn localkspace() -> Result<(), Box<dyn Error>> {
    let pos = parse_args()?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(io::stdin());

    let d: Vec<_> = rdr
        .headers()?
        .iter()
        .map(|h| EncodingField::from_name(h).unwrap().derivative_at(&pos))
        .collect();

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for sample in rdr.deserialize() {
        let s: Vec<f64> = sample?;
        let out: Vec<f64> = (0..=1)
            .map(|i| d.iter().zip(s.iter()).map(|(dd, ss)| dd[i] * ss).sum())
            .collect();

        wtr.serialize(out)?;
    }
    wtr.flush()?;
    Ok(())
}

fn parse_args() -> Result<[f64; 2], Box<dyn Error>> {
    let o: Vec<_> = env::args_os()
        .skip(1)
        .map(|x| {
            x.into_string()
                .expect("Argument not a valid string")
                .parse::<f64>()
                .expect("Argument does not seem to be a number")
        })
        .collect();
    Ok([o[0], o[1]])
}

fn main() {
    if let Err(err) = localkspace() {
        println!("Error computing local k space: {}", err);
        process::exit(1);
    }
}
