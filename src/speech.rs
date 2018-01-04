use monster;
use rand::{self, Rng, thread_rng};
use util::{pick};
use regex::{Captures, Regex};

const INTRO_LINES: &'static [&'static str] = &[
    "Anxious but determined, you begin your quest.",
    "Welcome to the Cellar. Will you prevail or perish?",
    "Dive down, nab the pendant, abscond. Let's do this.",
];

// generate("a{b|c}d") returns "abd" or "acd" randomly.
fn generate(pattern: &str) -> String {
    let re = Regex::new(r"\{([^\{\}]+)\}").unwrap();
    let mut s = pattern.to_string();
    while s.contains('{') {
        s = re.replace_all(&s, |c: &Captures| {
            pick(c[1].split('|')).to_string()
        }).to_string();
    }
    s
}

pub fn intro_line() -> String {
    generate(pick(INTRO_LINES))
}

pub fn shout_line(mk: monster::Kind) -> String {
    use monster::Kind::*;
    generate(match mk {
        Kestrel => "The kestrel {screeches|caws|shrieks}!",
        Skeleton => "The skeleton rattles!",
        Troll => "The troll {grunts|bellows}!",
        Android => "The android {{beeps|shouts} at you|sounds an alarm}!",
        Jelly => "The jelly makes a loud, squishy sound!",
        Salamander => "The salamander hisses!",
        TinyUFO => "The tiny UFO bleeps a {brief|frenzied} melody!",
        Minotaur => "The minotaur {snorts|huffs}!",
        Glitch => "The glitch emits {a loud sine wave|crashing white noise|jarring beeps}!",
        Witch => "The witch yells, \"{Hey, you!|Who goes there?!|Die, thief!}\"",
        Ghost => "The dungeon {rattles|shakes|tremors} around the ghost.",
        Soldier => "The soldier shouts, \"{Halt|Hold it|Freeze}!\"",
        Attractor => "The attractor sounds a {deep|piercing|mysterious|low} hum.",
        Turret => "The turret's servos whirr rapidly!",
        Elf => "(You should never see this.)", // Because your allies never shout.
        GoldenDragon => "The golden dragon blasts {a prismatic|an astral|a luminous} roar!",
    })
}

pub fn combat_line(mk: monster::Kind) -> String {
    use monster::Kind::*;
    generate(match mk {
        Kestrel => "The kestrel flutters around.",
        _ => "(more monster combat prose here)",
    })
}
