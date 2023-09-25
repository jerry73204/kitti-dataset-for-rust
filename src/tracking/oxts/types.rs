use crate::serde::{mode, u8_as_f64};
use measurements::{Acceleration, Angle, AngularVelocity, Length, Speed};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedOxts {
    /// latitude of the oxts-unit (deg)
    pub lat: f64,

    /// longitude of the oxts-unit (deg)
    pub lon: f64,

    /// altitude of the oxts-unit (m)
    pub alt: f64,

    /// roll angle (rad),  0 = level, positive = left side up (-pi..pi)
    pub roll: f64,

    /// pitch angle (rad), 0 = level, positive = front down (-pi/2..pi/2)
    pub pitch: f64,

    /// heading (rad),     0 = east,  positive = counter clockwise (-pi..pi)
    pub yaw: f64,

    /// velocity towards north (m/s)
    pub vn: f64,

    /// velocity towards east (m/s)
    pub ve: f64,

    /// forward velocity, i.e. parallel to earth-surface (m/s)
    pub vf: f64,

    /// leftward velocity, i.e. parallel to earth-surface (m/s)
    pub vl: f64,

    /// upward velocity, i.e. perpendicular to earth-surface (m/s)
    pub vu: f64,

    /// acceleration in x, i.e. in direction of vehicle front (m/s^2)
    pub ax: f64,

    /// acceleration in y, i.e. in direction of vehicle left (m/s^2)
    pub ay: f64,

    /// acceleration in z, i.e. in direction of vehicle top (m/s^2)
    pub az: f64,

    /// forward acceleration (m/s^2)
    pub af: f64,

    /// leftward acceleration (m/s^2)
    pub al: f64,

    /// upward acceleration (m/s^2)
    pub au: f64,

    /// angular rate around x (rad/s)
    pub wx: f64,

    /// angular rate around y (rad/s)
    pub wy: f64,

    /// angular rate around z (rad/s)
    pub wz: f64,

    /// angular rate around forward axis (rad/s)
    pub wf: f64,

    /// angular rate around leftward axis (rad/s)
    pub wl: f64,

    /// angular rate around upward axis (rad/s)
    pub wu: f64,

    /// velocity accuracy (north/east in m)
    pub posacc: f64,

    /// velocity accuracy (north/east in m/s)
    pub velacc: f64,

    /// navigation status
    #[serde(with = "u8_as_f64")]
    pub navstat: u8,

    /// number of satellites tracked by primary GPS receiver
    #[serde(with = "u8_as_f64")]
    pub numsats: u8,

    /// position mode of primary GPS receiver
    #[serde(with = "mode")]
    pub posmode: Option<u8>,

    /// velocity mode of primary GPS receiver
    #[serde(with = "mode")]
    pub velmode: Option<u8>,

    /// orientation mode of primary GPS receiver
    #[serde(with = "mode")]
    pub orimode: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "SerializedOxts", into = "SerializedOxts")]
pub struct Oxts {
    pub position: Position,
    pub rotation: Rotation,
    pub velocity: VelocityMeasurements,
    pub acceleration: AccelerationMeasurements,
    pub angular_velocity: AngularVelocityMeasurements,
    pub accuracy: Accuracy,
    pub gps_status: GpsStatus,
}

#[derive(Debug, Clone)]
pub struct Position {
    /// latitude of the oxts-unit (deg)
    pub lat: Angle,

    /// longitude of the oxts-unit (deg)
    pub lon: Angle,

    /// altitude of the oxts-unit (m)
    pub alt: Length,
}

#[derive(Debug, Clone)]
pub struct Rotation {
    /// roll angle (rad),  0 = level, positive = left side up (-pi..pi)
    pub roll: Angle,

    /// pitch angle (rad), 0 = level, positive = front down (-pi/2..pi/2)
    pub pitch: Angle,

    /// heading (rad),     0 = east,  positive = counter clockwise (-pi..pi)
    pub yaw: Angle,
}

#[derive(Debug, Clone)]
pub struct VelocityMeasurements {
    /// velocity towards north (m/s)
    pub vn: Speed,

    /// velocity towards east (m/s)
    pub ve: Speed,

    /// forward velocity, i.e. parallel to earth-surface (m/s)
    pub vf: Speed,

    /// leftward velocity, i.e. parallel to earth-surface (m/s)
    pub vl: Speed,

    /// upward velocity, i.e. perpendicular to earth-surface (m/s)
    pub vu: Speed,
}

#[derive(Debug, Clone)]
pub struct AccelerationMeasurements {
    /// acceleration in x, i.e. in direction of vehicle front (m/s^2)
    pub ax: Acceleration,

    /// acceleration in y, i.e. in direction of vehicle left (m/s^2)
    pub ay: Acceleration,

    /// acceleration in z, i.e. in direction of vehicle top (m/s^2)
    pub az: Acceleration,

    /// forward acceleration (m/s^2)
    pub af: Acceleration,

    /// leftward acceleration (m/s^2)
    pub al: Acceleration,

    /// upward acceleration (m/s^2)
    pub au: Acceleration,
}

#[derive(Debug, Clone)]
pub struct AngularVelocityMeasurements {
    /// angular rate around x (rad/s)
    pub wx: AngularVelocity,

    /// angular rate around y (rad/s)
    pub wy: AngularVelocity,

    /// angular rate around z (rad/s)
    pub wz: AngularVelocity,

    /// angular rate around forward axis (rad/s)
    pub wf: AngularVelocity,

    /// angular rate around leftward axis (rad/s)
    pub wl: AngularVelocity,

    /// angular rate around upward axis (rad/s)
    pub wu: AngularVelocity,
}

#[derive(Debug, Clone)]
pub struct Accuracy {
    /// position accuracy (north/east in m)
    pub posacc: Length,

    /// velocity accuracy (north/east in m/s)
    pub velacc: Speed,
}

#[derive(Debug, Clone)]
pub struct GpsStatus {
    /// navigation status
    pub navstat: u8,

    /// number of satellites tracked by primary GPS receiver
    pub numsats: u8,

    /// position mode of primary GPS receiver
    pub posmode: Option<u8>,

    /// velocity mode of primary GPS receiver
    pub velmode: Option<u8>,

    /// orientation mode of primary GPS receiver
    pub orimode: Option<u8>,
}

impl From<SerializedOxts> for Oxts {
    fn from(from: SerializedOxts) -> Self {
        let SerializedOxts {
            lat,
            lon,
            alt,
            roll,
            pitch,
            yaw,
            vn,
            ve,
            vf,
            vl,
            vu,
            ax,
            ay,
            az,
            af,
            al,
            au,
            wx,
            wy,
            wz,
            wf,
            wl,
            wu,
            posacc,
            velacc,
            navstat,
            numsats,
            posmode,
            velmode,
            orimode,
        } = from;

        Oxts {
            position: Position {
                lat: Angle::from_degrees(lat),
                lon: Angle::from_degrees(lon),
                alt: Length::from_meters(alt),
            },
            rotation: Rotation {
                roll: Angle::from_radians(roll),
                pitch: Angle::from_radians(pitch),
                yaw: Angle::from_radians(yaw),
            },
            velocity: VelocityMeasurements {
                vn: Speed::from_meters_per_second(vn),
                ve: Speed::from_meters_per_second(ve),
                vf: Speed::from_meters_per_second(vf),
                vl: Speed::from_meters_per_second(vl),
                vu: Speed::from_meters_per_second(vu),
            },
            acceleration: AccelerationMeasurements {
                ax: Acceleration::from_meters_per_second_per_second(ax),
                ay: Acceleration::from_meters_per_second_per_second(ay),
                az: Acceleration::from_meters_per_second_per_second(az),
                af: Acceleration::from_meters_per_second_per_second(af),
                al: Acceleration::from_meters_per_second_per_second(al),
                au: Acceleration::from_meters_per_second_per_second(au),
            },
            angular_velocity: AngularVelocityMeasurements {
                wx: AngularVelocity::from_radians_per_second(wx),
                wy: AngularVelocity::from_radians_per_second(wy),
                wz: AngularVelocity::from_radians_per_second(wz),
                wf: AngularVelocity::from_radians_per_second(wf),
                wl: AngularVelocity::from_radians_per_second(wl),
                wu: AngularVelocity::from_radians_per_second(wu),
            },
            accuracy: Accuracy {
                posacc: Length::from_meters(posacc),
                velacc: Speed::from_meters_per_second(velacc),
            },
            gps_status: GpsStatus {
                navstat,
                numsats,
                posmode,
                velmode,
                orimode,
            },
        }
    }
}

impl From<Oxts> for SerializedOxts {
    fn from(from: Oxts) -> Self {
        let Oxts {
            position: Position { lat, lon, alt },
            rotation: Rotation { roll, pitch, yaw },
            velocity: VelocityMeasurements { vn, ve, vf, vl, vu },
            acceleration:
                AccelerationMeasurements {
                    ax,
                    ay,
                    az,
                    af,
                    al,
                    au,
                },
            angular_velocity:
                AngularVelocityMeasurements {
                    wx,
                    wy,
                    wz,
                    wf,
                    wl,
                    wu,
                },
            accuracy: Accuracy { posacc, velacc },
            gps_status:
                GpsStatus {
                    navstat,
                    numsats,
                    posmode,
                    velmode,
                    orimode,
                },
        } = from;

        Self {
            lat: lat.as_degrees(),
            lon: lon.as_degrees(),
            alt: alt.as_meters(),
            roll: roll.as_radians(),
            pitch: pitch.as_radians(),
            yaw: yaw.as_radians(),
            vn: vn.as_meters_per_second(),
            ve: ve.as_meters_per_second(),
            vf: vf.as_meters_per_second(),
            vl: vl.as_meters_per_second(),
            vu: vu.as_meters_per_second(),
            ax: ax.as_meters_per_second_per_second(),
            ay: ay.as_meters_per_second_per_second(),
            az: az.as_meters_per_second_per_second(),
            af: af.as_meters_per_second_per_second(),
            al: al.as_meters_per_second_per_second(),
            au: au.as_meters_per_second_per_second(),
            wx: wx.as_radians_per_second(),
            wy: wy.as_radians_per_second(),
            wz: wz.as_radians_per_second(),
            wf: wf.as_radians_per_second(),
            wl: wl.as_radians_per_second(),
            wu: wu.as_radians_per_second(),
            posacc: posacc.as_meters(),
            velacc: velacc.as_meters_per_second(),
            navstat,
            numsats,
            posmode,
            velmode,
            orimode,
        }
    }
}
