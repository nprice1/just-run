use std::ops::{Deref, Neg, Add, Sub, Mul, Div};

use std::f64;
use super::drawing::{Game}; 

pub trait AsFloat {
	fn as_f64(self)   ->  f64;
	fn as_nt(val: f64) -> Self;
}

/// Millis represents a length of time in milliseconds as a signed integer.
/// (NOTE: As `Millis` supports basic arithmetic: "negative time" is possible.)
#[derive(Eq,Ord,PartialEq,PartialOrd,Clone,Copy)]
pub struct Millis(pub i64);

impl Add<Millis> for Millis {
	type Output = Millis;

	#[inline]
	fn add(self, rhs: Millis) -> Millis {
		let (Millis(t0), Millis(t1)) = (self, rhs);
		Millis(t0 + t1)
	}
}

impl Sub<Millis> for Millis {
	type Output = Millis;

	#[inline]
	fn sub(self, rhs: Millis) -> Millis {
		let (Millis(t0), Millis(t1)) = (self, rhs);
		Millis(t0 - t1)
	}
}

/// Velocity represents the current speed of an object.
/// This speed is measured in Games/Millis, and is stored as a float.
///
/// (Note: this is actually `Pixels/ms`, but `Games` are used as
/// they are higher precision types, they will also automatically
/// scale the render distance when converted to pixels.)
#[derive(PartialEq,PartialOrd,Clone,Copy)]
pub struct Velocity(pub f64);

impl AsFloat for Velocity {
	#[inline]
	fn as_f64(self) -> f64 {
		let Velocity(v0) = self;
		return v0;
	}

	#[inline]
	fn as_nt(val: f64) -> Velocity { Velocity(val) }
}

impl Neg for Velocity {
	type Output = Velocity;

	#[inline]
	fn neg(self) -> Velocity {
		let Velocity(v0) = self;
		Velocity(-v0)
	}
}

impl Add<Velocity> for Velocity {
	type Output = Velocity;

	#[inline]
	fn add(self, rhs: Velocity) -> Velocity {
		let (Velocity(v0), Velocity(v1)) = (self, rhs);
		Velocity(v0 + v1)
	}
}

impl Sub<Velocity> for Velocity {
	type Output = Velocity;

	#[inline]
	fn sub(self, rhs: Velocity) -> Velocity {
		let (Velocity(v0), Velocity(v1)) = (self, rhs);
		Velocity(v0 - v1)
	}
}

/// Any velocity multiplied by some length in time `t`
/// results in a distance measured in `Games`
impl Mul<Millis> for Velocity {
	type Output = Game;

	#[inline]
	fn mul(self, rhs: Millis) -> Game {
		let (Velocity(v0), Millis(t)) = (self, rhs);
		Game(v0 * t as f64)
	}
}

/// Acceleration is defined as `(Games/ms)/ms`
#[derive(PartialEq,PartialOrd,Clone,Copy)]
pub struct Acceleration(pub f64);

/// Acceleration `a` multipled by some time `t` results
/// in `Velocity(a * t)`
impl Mul<Millis> for Acceleration {
	type Output = Velocity;

	#[inline]
	fn mul(self, rhs: Millis) -> Velocity {
		let (Acceleration(a), Millis(t)) = (self, rhs);
		Velocity(a * t as f64)
	}
}

impl Neg for Acceleration {
	type Output = Acceleration;

	#[inline]
	fn neg(self) -> Acceleration {
		let Acceleration(a) = self;
		Acceleration(-a)
	}
}

#[derive(PartialEq,PartialOrd,Clone,Copy)]
pub struct Degrees(pub f64);

impl Degrees {

	/// Degrees are converted to radians as follows: `Degrees * (PI / 180.0)`
	pub fn to_radians(self) -> f64 {
		let Degrees(d) = self;
		d * (f64::consts::PI / 180.0)
	}
}

impl Add<Degrees> for Degrees {
	type Output = Degrees;

	#[inline]
	fn add(self, rhs: Degrees) -> Degrees {
		let (Degrees(d0), Degrees(d1)) = (self, rhs);
		Degrees(d0 + d1)
	}
}

/// Some number of Degrees `d` divided by some time `t` yields
/// an AngularVelocity `av`
impl Div<Millis> for Degrees {
	type Output = AngularVelocity;

	#[inline]
	fn div(self, rhs: Millis) -> AngularVelocity {
		let (Degrees(d), Millis(t)) = (self, rhs);
		AngularVelocity(d / t as f64)
	}
}

/// AngularVelocity is defined as `Degrees/Millis` and is stored in a float.
#[derive(PartialEq,PartialOrd,Clone,Copy)]
pub struct AngularVelocity(pub f64);

/// Some AngularVelocity `av` multiplied by some time `t` yields
/// a number of degrees `d`.
impl Mul<Millis> for AngularVelocity {
	type Output = Degrees;

	#[inline]
	fn mul(self, rhs: Millis) -> Degrees {
		let (AngularVelocity(av), Millis(t)) = (self, rhs);
		Degrees(av * t as f64)
	}
}

pub fn min<T: AsFloat>(lhs: T, rhs: T) -> T {
	AsFloat::as_nt(lhs.as_f64().min(rhs.as_f64()))
}

pub fn max<T: AsFloat>(lhs: T, rhs: T) -> T {
	AsFloat::as_nt(lhs.as_f64().max(rhs.as_f64()))
}

pub type Frame = u32;
pub type Fps = u32;
