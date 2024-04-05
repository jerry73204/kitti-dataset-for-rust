use crate::{
    common::{ProjectionMatrix, Transform2D},
    Error,
};
use itertools::Itertools;
use slice_of_array::prelude::*;
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, prelude::*, BufReader, Cursor},
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

macro_rules! make_struct {
    ( $name:ident {$($body:tt)*} ($field:ident : $ty:ident) $($tail:tt)* ) => {
        make_struct! {
            $name
            {
                $($body)*
                pub $field : $ty,
            }
            $($tail)*
        }
    };

    ( $name:ident {$($body:tt)*} ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($body)*
        }
    };
}

macro_rules! read_field {
    ( $lines:ident { $($body:tt)* } ($name:expr, $field:ident, ProjectionMatrix) $($tail:tt)* ) => {
        read_field! {
            $lines
            {
                $($body)*
                $field: {
                    let tokens = next_line(&mut $lines, $name, 12)?;
                    let value = to_projection_matrix(tokens);
                    value
                },
            }
            $($tail)*
        }
    };
    ( $lines:ident { $($body:tt)* } ($name:expr, $field:ident, Transform2D) $($tail:tt)* ) => {
        read_field! {
            $lines
            {
                $($body)*
                $field: {
                    let tokens = next_line(&mut $lines, $name, 9)?;
                    let value = to_rect_matrix(tokens);
                    value
                },
            }
            $($tail)*
        }
    };
    ( $lines:ident { $($body:tt)* } ) => {
        Self {
            $($body)*
        }
    };
}

macro_rules! write_field {
    ( $self:ident $f:ident { $($body:tt)* } ($prefix:expr, $field:ident, ProjectionMatrix) $($tail:tt)* ) => {
        write_field! {
            $self
            $f
            {
                $($body)*
                write_line(
                    $f,
                    $prefix,
                    & $self.$field.0,
                )?;

            }
            $($tail)*
        }
    };
    ( $self:ident $f:ident { $($body:tt)* } ($prefix:expr, $field:ident, Transform2D) $($tail:tt)* ) => {
        write_field! {
            $self
            $f
            {
                $($body)*
                write_line(
                    $f,
                    $prefix,
                    & $self.$field.0,
                )?;
            }
            $($tail)*
        }
    };
    ( $self:ident $f:ident { $($body:tt)* } ) => {
        $($body)*
    };
}

macro_rules! generate_calib_type {
    ($name:ident; $( ($prefix:expr, $field:ident, $ty:ident) ),*  ) => {
        make_struct! {
            $name
            {}
            $( ($field : $ty) )*
        }

        impl $name {
            pub fn from_reader<R>(reader: R) -> Result<Self, Error>
            where
                R: Read
            {
                let mut lines = reader_to_lines(reader);

                Ok(
                    read_field! {
                        lines
                        {}
                        $( ($prefix, $field, $ty) )*
                    }

                )
            }

            pub fn from_path<P>(path: P) -> Result<Self, Error>
            where
                P: AsRef<Path>
            {
                let reader = BufReader::new(File::open(path)?);
                Self::from_reader(reader)
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                let reader = Cursor::new(text);
                Self::from_reader(reader)
            }
        }



        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write_field! {
                    self
                    f
                    {}
                    $( ($prefix, $field, $ty) )*
                }

                Ok(())
            }
        }
    };
}

generate_calib_type! {
    ObjectCalibration;
    ("P0:", p0, ProjectionMatrix),
    ("P1:", p1, ProjectionMatrix),
    ("P2:", p2, ProjectionMatrix),
    ("P3:", p3, ProjectionMatrix),
    ("R0_rect:", r0_rect, Transform2D),
    ("Tr_velo_to_cam:", tr_velo_to_cam, ProjectionMatrix),
    ("Tr_imu_to_velo:", tr_imu_to_velo, ProjectionMatrix)
}

generate_calib_type! {
    TrackingCalibration;
    ("P0:", p0, ProjectionMatrix),
    ("P1:", p1, ProjectionMatrix),
    ("P2:", p2, ProjectionMatrix),
    ("P3:", p3, ProjectionMatrix),
    ("R_rect", r0_rect, Transform2D),
    ("Tr_velo_cam", tr_velo_to_cam, ProjectionMatrix),
    ("Tr_imu_velo", tr_imu_to_velo, ProjectionMatrix)
}

generate_calib_type! {
    OdometryCalibration;
    ("P0:", p0, ProjectionMatrix),
    ("P1:", p1, ProjectionMatrix),
    ("P2:", p2, ProjectionMatrix),
    ("P3:", p3, ProjectionMatrix),
    ("Tr:", tr, ProjectionMatrix)
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
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "unexpected end of file in calib config",
        )
        .into());
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

fn reader_to_lines<R: Read>(reader: R) -> impl Iterator<Item = Result<CalibLine, Error>> {
    BufReader::new(reader)
        .lines()
        .map(|line| -> Result<_, Error> {
            let line = line?;
            let line = line.trim();
            let mut tokens = line.split_ascii_whitespace();

            let Some(name) = tokens.next() else {
                bail!("unexpected empty line");
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

fn write_line<const M: usize, const N: usize>(
    f: &mut Formatter<'_>,
    name: &str,
    array: &[[f32; N]; M],
) -> Result<(), fmt::Error> {
    write!(f, "{name}:")?;

    let slice = array.flat();
    for &val in slice {
        write!(f, " {val:.12e}")?;
    }

    writeln!(f)?;
    Ok(())
}
