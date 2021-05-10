//! A tiny, robust PRNG implementation.
//!
//! More specifically, it implements a single GOOD PRNG algorithm,
//! which is currently a permuted congruential generator.  It has two
//! implementations, one that returns `u32` and one that returns
//! `u64`.  It also has functions that return floats or integer
//! ranges.  And that's it.  What more do you need?
//!
//! For more info on PCG generators, see http://www.pcg-random.org/
//!
//! This was designed as a minimalist utility for video games.  No
//! promises are made about its quality, and if you use it for
//! cryptography you will get what you deserve.
//!
//! Works with `#![no_std]`, has no global state, no dependencies
//! apart from some in the unit tests, and is generally neato.

// #![forbid(unsafe_code)]
// #![forbid(missing_docs)]
// #![forbid(missing_debug_implementations)]
// #![forbid(unused_results)]
use std::sync::atomic::{AtomicU64, Ordering};

/// A PRNG producing a 32-bit output.
///
/// The current implementation is `PCG-XSH-RR`.
#[derive(Debug)]
pub struct Random {
    state: AtomicU64,
    inc: AtomicU64,
}

impl Random {
    /// The default value for `increment`.
    /// This is basically arbitrary, it comes from the
    /// PCG reference C implementation:
    /// https://github.com/imneme/pcg-c/blob/master/include/pcg_variants.h#L284
    pub const DEFAULT_INC: u64 = 1442695040888963407;

    /// This is the number that you have to Really Get Right.
    ///
    /// The value used here is from the PCG C implementation:
    /// https://github.com/imneme/pcg-c/blob/master/include/pcg_variants.h#L278
    pub(crate) const MULTIPLIER: u64 = 6364136223846793005;

    /// Creates a new PRNG with the given seed and a default increment.
    pub const fn new() -> Self {
        Self {
            state: AtomicU64::new(0),
            inc: AtomicU64::new(1),
        }
    }

    /// Calls increment function with default
    /// See docs there
    pub fn seed(&self, seed: u64) {
        self.increment(seed, Self::DEFAULT_INC);
    }

    /// Creates a new PRNG.  The two inputs, `seed` and `increment`,
    /// determine what you get; `increment` basically selects which
    /// sequence of all those possible the PRNG will produce, and the
    /// `seed` selects where in that sequence you start.
    ///
    /// Both are arbitrary; increment must be an odd number but this
    /// handles that for you
    pub fn increment(&self, seed: u64, increment: u64) {
        self.inc.store(increment.wrapping_shl(1) | 1, Ordering::Relaxed);
        // This initialization song-and-dance is a little odd,
        // but seems to be just how things go.
        let _ = self.rand();
        self.state.store(self.state.load(Ordering::Relaxed).wrapping_add(seed), Ordering::Relaxed);
        let _ = self.rand();
    }

    /// Returns the internal state of the PRNG.  This allows
    /// you to save a PRNG and create a new one that will resume
    /// from the same spot in the sequence.
    pub fn state(&self) -> (u64, u64) {
        (self.state.load(Ordering::Relaxed), self.inc.load(Ordering::Relaxed))
    }

    /// Creates a new PRNG from a saved state from `Rand32::state()`.
    /// This is NOT quite the same as `new_inc()` because `new_inc()` does
    /// a little extra setup work to initialize the state.
    // pub fn from_state(state: (u64, u64)) -> Self {
    //     let (state, inc) = state;
    //     Self { state, inc }
    // }

    /// Produces a random `u32` in the range `[0, u32::MAX]`.
    pub fn rand(&self) -> u32 {
        let oldstate: u64 = self.state.load(Ordering::Relaxed);
        self.state.store(oldstate
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(self.inc.load(Ordering::Relaxed)), Ordering::Relaxed);
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: u32 = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    /// Produces a random `i32` in the range `[i32::MIN, i32::MAX]`.
    pub fn gen_i32(&self) -> i32 {
        self.rand() as i32
    }

    /// Produces a random `f32` in the range `[0.0, 1.0)`.
    pub fn gen_float(&self) -> f32 {
        // This impl was taken more or less from `rand`, see
        // <https://docs.rs/rand/0.7.0/src/rand/distributions/float.rs.html#104-117>
        // There MAY be better ways to do this, see:
        // https://mumble.net/~campbell/2014/04/28/uniform-random-float
        // https://mumble.net/~campbell/2014/04/28/random_real.c
        // https://github.com/Lokathor/randomize/issues/34
        const TOTAL_BITS: u32 = 32;
        const PRECISION: u32 = core::f32::MANTISSA_DIGITS + 1;
        const MANTISSA_SCALE: f32 = 1.0 / ((1u32 << PRECISION) as f32);
        let mut u = self.rand();
        u >>= TOTAL_BITS - PRECISION;
        u as f32 * MANTISSA_SCALE
    }

    /// Produces a random within the given bounds.  Like any `Range`,
    /// it includes the lower bound and excludes the upper one.
    ///
    /// This should be faster than `Self::rand() % end + start`, but the
    /// real advantage is it's more convenient.  Requires that
    /// `range.end <= range.start`.
    pub fn gen_range<N: IntoRandNum>(&self, min: N, max: N) -> N {
        // This is harder to do well than it looks, it seems.  I don't
        // trust Lokathor's implementation 'cause I don't understand
        // it, so I went to numpy's, which points me to "Lemire's
        // rejection algorithm": http://arxiv.org/abs/1805.10941
        //
        // Algorithms 3, 4 and 5 in that paper all seem fine modulo
        // minor performance differences, so this is algorithm 5.
        // It uses numpy's implementation, `buffered_bounded_lemire_uint32()`

        let range = min.into_u32()..max.into_u32();

        debug_assert!(range.start < range.end);
        let range_starting_from_zero = 0..(range.end - range.start);

        let s: u32 = range_starting_from_zero.end;
        let mut m: u64 = u64::from(self.rand()) * u64::from(s);
        let mut leftover: u32 = (m & 0xFFFF_FFFF) as u32;

        if leftover < s {
            // TODO: verify the wrapping_neg() here
            let threshold: u32 = s.wrapping_neg() % s;
            while leftover < threshold {
                m = u64::from(self.rand()).wrapping_mul(u64::from(s));
                leftover = (m & 0xFFFF_FFFF) as u32;
            }
        }
        N::from_u32((m >> 32) as u32 + range.start)
    }

    pub fn gen_bool(&self) -> bool {
        self.gen_range(0u32, 2) == 0
    }

}

pub trait IntoRandNum {

    fn into_u32(self) -> u32;

    fn from_u32(num: u32) -> Self;

}

impl IntoRandNum for u8 {
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_u32(num: u32) -> Self {
        num as Self
    }
}

impl IntoRandNum for u16 {
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_u32(num: u32) -> Self {
        num as Self
    }
}

impl IntoRandNum for u32 {
    fn into_u32(self) -> u32 {
        self
    }
    fn from_u32(num: u32) -> Self {
        num
    }
}

impl IntoRandNum for usize {
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_u32(num: u32) -> Self {
        num as Self
    }
}