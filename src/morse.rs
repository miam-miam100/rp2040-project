use cortex_m::delay::Delay;
use embedded_hal::PwmPin;
use rp_pico::hal::pwm;
use rp_pico::hal::pwm::{Channel, SliceId, SliceMode};

/// The unit of time a dot should be on for.
pub const UNIT: u32 = 200;
/// The tone to play at
pub const TONE: u16 = 10000;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Signal {
    Dot,
    Dash,
}

macro_rules! morse {
    ($($sig: tt)*) => {
        &[$(signal!($sig),)*]
    };
}

// Need to add spaces between .. and ... as they match rust syntax and get tokenized differently
macro_rules! signal {
    (.) => {
        Signal::Dot
    };
    (-) => {
        Signal::Dash
    };
}

const ALPHAS: [&[Signal]; 26] = [
    morse!(.-),
    morse!(-. . .),
    morse!(-.-.),
    morse!(-. .),
    morse!(.),
    morse!(. .-.),
    morse!(--.),
    morse!(. . . .),
    morse!(. .),
    morse!(.---),
    morse!(-.-),
    morse!(.-. .),
    morse!(--),
    morse!(-.),
    morse!(---),
    morse!(.--.),
    morse!(--.-),
    morse!(.-.),
    morse!(. . .),
    morse!(-),
    morse!(. .-),
    morse!(. . .-),
    morse!(.--),
    morse!(-. .-),
    morse!(-.--),
    morse!(--. .),
];

const NUMBERS: [&[Signal]; 10] = [
    morse!(-----),
    morse!(.----),
    morse!(. .---),
    morse!(. . .--),
    morse!(. . . .-),
    morse!(. . . . .),
    morse!(-. . . .),
    morse!(--. . .),
    morse!(---. .),
    morse!(----.),
];

pub struct MorseCode<'a, S: SliceId, M: SliceMode> {
    buf: &'a [u8],
    channel: &'a mut Channel<S, M, pwm::A>,
}

impl<'a, S, M> MorseCode<'a, S, M>
where
    S: SliceId,
    M: SliceMode,
{
    pub fn new(buffer: &'a [u8], channel: &'a mut Channel<S, M, pwm::A>) -> Self {
        Self {
            buf: buffer,
            channel,
        }
    }

    pub fn get_char(&self) -> Option<char> {
        if self.buf.is_empty() {
            None
        } else {
            match self.buf[0].to_ascii_uppercase().into() {
                c @ ('0'..='9' | 'A'..='Z' | 'a'..='z' | ' ') => Some(c),
                _ => None,
            }
        }
    }

    // https://morsecode.world/international/timing.html
    pub fn consume_tone(&mut self, delay: &mut Delay) {
        let signals = match self.get_char() {
            Some(c @ '0'..='9') => NUMBERS[(c as u8 - b'0') as usize],
            Some(c @ 'A'..='Z') => ALPHAS[(c as u8 - b'A') as usize],
            Some(c @ 'a'..='z') => ALPHAS[(c as u8 - b'a') as usize],
            Some(' ') => {
                delay.delay_ms(UNIT * 7);
                self.buf = &self.buf[1..];
                return;
            }
            None | Some(_) => {
                return;
            }
        };
        self.buf = &self.buf[1..];
        for (i, signal) in signals.iter().enumerate() {
            match signal {
                Signal::Dot => {
                    self.channel.enable();
                    delay.delay_ms(UNIT);
                    self.channel.disable();
                }
                Signal::Dash => {
                    self.channel.enable();
                    delay.delay_ms(UNIT * 3);
                    self.channel.disable();
                }
            }
            if i + 1 != signals.len() {
                delay.delay_ms(UNIT);
            }
        }
    }
}
