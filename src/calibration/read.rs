use super::{CameraCalibration, OdometryCalibration, ProjectionMatrix, Transform2D};
use crate::Error;
use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read},
    path::Path,
    str::FromStr,
};

macro_rules! error {
    ($format:expr $(,$arg:expr)*) => {
        Error::InvalidCalibConfig(format!($format $(,$arg)*))
    };
}

macro_rules! bail {
    ($format:expr $(,$arg:expr)*) => {
        return Err(error!($format $(,$arg)*));
    };
}

macro_rules! ensure {
    ($cond:expr, $format:expr $(,$arg:expr)*) => {
        if !$cond {
            bail!($format $(,$arg)*);
        }
    };
}

impl CameraCalibration {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let mut lines = parse_lines(reader);

        let p0 = next_line(&mut lines, "P0", 12)?;
        let p1 = next_line(&mut lines, "P1", 12)?;
        let p2 = next_line(&mut lines, "P2", 12)?;
        let p3 = next_line(&mut lines, "P3", 12)?;
        let r0_rect = next_line(&mut lines, "R0_rect", 9)?;
        let tr_velo_to_cam = next_line(&mut lines, "Tr_velo_to_cam", 12)?;
        let tr_imu_to_velo = next_line(&mut lines, "Tr_imu_to_velo", 12)?;

        let p0 = to_projection_matrix(p0);
        let p1 = to_projection_matrix(p1);
        let p2 = to_projection_matrix(p2);
        let p3 = to_projection_matrix(p3);
        let r0_rect = to_rect_matrix(r0_rect);
        let tr_velo_to_cam = to_projection_matrix(tr_velo_to_cam);
        let tr_imu_to_velo = to_projection_matrix(tr_imu_to_velo);

        Ok(Self {
            p0,
            p1,
            p2,
            p3,
            r0_rect,
            tr_velo_to_cam,
            tr_imu_to_velo,
        })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let reader = BufReader::new(File::open(path)?);
        Self::from_reader(reader)
    }
}

impl FromStr for CameraCalibration {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let reader = Cursor::new(text);
        Self::from_reader(reader)
    }
}

impl OdometryCalibration {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let mut lines = parse_lines(reader);

        let p0 = next_line(&mut lines, "P0", 12)?;
        let p1 = next_line(&mut lines, "P1", 12)?;
        let p2 = next_line(&mut lines, "P2", 12)?;
        let p3 = next_line(&mut lines, "P3", 12)?;
        let tr = next_line(&mut lines, "Tr", 12)?;

        let p0 = to_projection_matrix(p0);
        let p1 = to_projection_matrix(p1);
        let p2 = to_projection_matrix(p2);
        let p3 = to_projection_matrix(p3);
        let tr = to_projection_matrix(tr);

        Ok(Self { p0, p1, p2, p3, tr })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let reader = BufReader::new(File::open(path)?);
        Self::from_reader(reader)
    }
}

impl FromStr for OdometryCalibration {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let reader = Cursor::new(text);
        Self::from_reader(reader)
    }
}

struct CalibLine {
    pub name: String,
    pub values: Vec<f32>,
}

fn next_line<I>(mut lines: I, expect_name: &str, n_values: usize) -> Result<Vec<f32>, Error>
where
    I: Iterator<Item = Result<CalibLine, Error>>,
{
    let Some(line) = lines.next() else {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected end of file in calib config").into());
    };

    let line = line?;
    let CalibLine { name, values } = line;
    ensure!(
        name == expect_name,
        r#"expect prefix "{expect_name}", but get "{name}""#
    );

    ensure!(
        values.len() == n_values,
        "expect {n_values} values, but get {} values",
        values.len()
    );

    Ok(values)
}

fn parse_lines<R: Read>(reader: R) -> impl Iterator<Item = Result<CalibLine, Error>> {
    BufReader::new(reader)
        .lines()
        .map(|line| -> Result<_, Error> {
            let line = line?;
            let mut tokens = line.split(' ');

            let Some(prefix) = tokens.next() else {
                bail!("unexpected empty line");
            };
            let Some(name) = prefix.strip_suffix(':') else {
                bail!("invalid calib format, expect a prefix with a colon.");
            };
            let values: Vec<f32> = tokens
                .map(|token| {
                    token
                        .parse()
                        .map_err(|_| error!(r#"invalid token "{token}" in line "{line}""#))
                })
                .try_collect()?;

            Ok(CalibLine {
                name: name.to_string(),
                values,
            })
        })
}

fn to_projection_matrix(params: Vec<f32>) -> ProjectionMatrix {
    let params: [f32; 12] = params.try_into().unwrap();
    let [r00, r01, r02, r03, r10, r11, r12, r13, r20, r21, r22, r23] = params;

    ProjectionMatrix([
        [r00, r01, r02, r03],
        [r10, r11, r12, r13],
        [r20, r21, r22, r23],
    ])
}

fn to_rect_matrix(params: Vec<f32>) -> Transform2D {
    let params: [f32; 9] = params.try_into().unwrap();
    let [p00, p01, p02, p10, p11, p12, p20, p21, p22] = params;
    Transform2D([[p00, p01, p02], [p10, p11, p12], [p20, p21, p22]])
}
