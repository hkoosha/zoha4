use std::fmt::{Display, Formatter};
use gdk4::RGBA;
use serde::Deserialize;

fn f(value: i32) -> f32 {
    return (value as f32) / (0xffff as f32);
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Pallet {
    Tango,
    Zenburn,
    Linux,
    XTerm,
    RXVT,
    SolarizedLight,
    SolarizedDark,
    Snazzy,
}

impl Display for Pallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pallet::Tango => write!(f, "Tango"),
            Pallet::Zenburn => write!(f, "Zenburn"),
            Pallet::Linux => write!(f, "Linux"),
            Pallet::XTerm => write!(f, "XTerm"),
            Pallet::RXVT => write!(f, "RXVT"),
            Pallet::SolarizedLight => write!(f, "SolarizedLight"),
            Pallet::SolarizedDark => write!(f, "SolarizedDark"),
            Pallet::Snazzy => write!(f, "Snazzy"),
        }
    }
}

impl Pallet {
    fn tango() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0x2e2e), f(0x3434), f(0x3636), 1.0),
            RGBA::new(f(0xcccc), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0x4e4e), f(0x9a9a), f(0x0606), 1.0),
            RGBA::new(f(0xc4c4), f(0xa0a0), f(0x0000), 1.0),
            RGBA::new(f(0x3434), f(0x6565), f(0xa4a4), 1.0),
            RGBA::new(f(0x7575), f(0x5050), f(0x7b7b), 1.0),
            RGBA::new(f(0x0606), f(0x9820), f(0x9a9a), 1.0),
            RGBA::new(f(0xd3d3), f(0xd7d7), f(0xcfcf), 1.0),
            RGBA::new(f(0x5555), f(0x5757), f(0x5353), 1.0),
            RGBA::new(f(0xefef), f(0x2929), f(0x2929), 1.0),
            RGBA::new(f(0x8a8a), f(0xe2e2), f(0x3434), 1.0),
            RGBA::new(f(0xfcfc), f(0xe9e9), f(0x4f4f), 1.0),
            RGBA::new(f(0x7272), f(0x9f9f), f(0xcfcf), 1.0),
            RGBA::new(f(0xadad), f(0x7f7f), f(0xa8a8), 1.0),
            RGBA::new(f(0x3434), f(0xe2e2), f(0xe2e2), 1.0),
            RGBA::new(f(0xeeee), f(0xeeee), f(0xecec), 1.0),
        ];
    }

    fn zenburn() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0x2222), f(0x2222), f(0x2222), 1.0), //black
            RGBA::new(f(0x8080), f(0x3232), f(0x3232), 1.0), //darkred
            RGBA::new(f(0x5b5b), f(0x7676), f(0x2f2f), 1.0), //darkgreen
            RGBA::new(f(0xaaaa), f(0x9999), f(0x4343), 1.0), //brown
            RGBA::new(f(0x3232), f(0x4c4c), f(0x8080), 1.0), //darkblue
            RGBA::new(f(0x7070), f(0x6c6c), f(0x9a9a), 1.0), //darkmagenta
            RGBA::new(f(0x9292), f(0xb1b1), f(0x9e9e), 1.0), //darkcyan
            RGBA::new(f(0xffff), f(0xffff), f(0xffff), 1.0), //lightgrey
            RGBA::new(f(0x2222), f(0x2222), f(0x2222), 1.0), //darkgrey
            RGBA::new(f(0x9898), f(0x2b2b), f(0x2b2b), 1.0), //red
            RGBA::new(f(0x8989), f(0xb8b8), f(0x3f3f), 1.0), //green
            RGBA::new(f(0xefef), f(0xefef), f(0x6060), 1.0), //yellow
            RGBA::new(f(0x2b2b), f(0x4f4f), f(0x9898), 1.0), //blue
            RGBA::new(f(0x8282), f(0x6a6a), f(0xb1b1), 1.0), //magenta
            RGBA::new(f(0xa1a1), f(0xcdcd), f(0xcdcd), 1.0), //cyan
            RGBA::new(f(0xdede), f(0xdede), f(0xdede), 1.0), //white
        ];
    }

    fn linux() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0x0000), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0xaaaa), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0xaaaa), f(0x0000), 1.0),
            RGBA::new(f(0xaaaa), f(0x5555), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0x0000), f(0xaaaa), 1.0),
            RGBA::new(f(0xaaaa), f(0x0000), f(0xaaaa), 1.0),
            RGBA::new(f(0x0000), f(0xaaaa), f(0xaaaa), 1.0),
            RGBA::new(f(0xaaaa), f(0xaaaa), f(0xaaaa), 1.0),
            RGBA::new(f(0x5555), f(0x5555), f(0x5555), 1.0),
            RGBA::new(f(0xffff), f(0x5555), f(0x5555), 1.0),
            RGBA::new(f(0x5555), f(0xffff), f(0x5555), 1.0),
            RGBA::new(f(0xffff), f(0xffff), f(0x5555), 1.0),
            RGBA::new(f(0x5555), f(0x5555), f(0xffff), 1.0),
            RGBA::new(f(0xffff), f(0x5555), f(0xffff), 1.0),
            RGBA::new(f(0x5555), f(0xffff), f(0xffff), 1.0),
            RGBA::new(f(0xffff), f(0xffff), f(0xffff), 1.0),
        ];
    }

    fn xterm() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0x0000), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0xcdcb), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0xcdcb), f(0x0000), 1.0),
            RGBA::new(f(0xcdcb), f(0xcdcb), f(0x0000), 1.0),
            RGBA::new(f(0x1e1a), f(0x908f), f(0xffff), 1.0),
            RGBA::new(f(0xcdcb), f(0x0000), f(0xcdcb), 1.0),
            RGBA::new(f(0x0000), f(0xcdcb), f(0xcdcb), 1.0),
            RGBA::new(f(0xe5e2), f(0xe5e2), f(0xe5e2), 1.0),
            RGBA::new(f(0x4ccc), f(0x4ccc), f(0x4ccc), 1.0),
            RGBA::new(f(0xffff), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0xffff), f(0x0000), 1.0),
            RGBA::new(f(0xffff), f(0xffff), f(0x0000), 1.0),
            RGBA::new(f(0x4645), f(0x8281), f(0xb4ae), 1.0),
            RGBA::new(f(0xffff), f(0x0000), f(0xffff), 1.0),
            RGBA::new(f(0x0000), f(0xffff), f(0xffff), 1.0),
            RGBA::new(f(0xffff), f(0xffff), f(0xffff), 1.0),
        ];
    }

    fn rxvt() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0x0000), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0xcdcd), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0xcdcd), f(0x0000), 1.0),
            RGBA::new(f(0xcdcd), f(0xcdcd), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0x0000), f(0xcdcd), 1.0),
            RGBA::new(f(0xcdcd), f(0x0000), f(0xcdcd), 1.0),
            RGBA::new(f(0x0000), f(0xcdcd), f(0xcdcd), 1.0),
            RGBA::new(f(0xfafa), f(0xebeb), f(0xd7d7), 1.0),
            RGBA::new(f(0x4040), f(0x4040), f(0x4040), 1.0),
            RGBA::new(f(0xffff), f(0x0000), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0xffff), f(0x0000), 1.0),
            RGBA::new(f(0xffff), f(0xffff), f(0x0000), 1.0),
            RGBA::new(f(0x0000), f(0x0000), f(0xffff), 1.0),
            RGBA::new(f(0xffff), f(0x0000), f(0xffff), 1.0),
            RGBA::new(f(0x0000), f(0xffff), f(0xffff), 1.0),
            RGBA::new(f(0xffff), f(0xffff), f(0xffff), 1.0),
        ];
    }

    fn solarized_light() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0xeeee), f(0xe8e8), f(0xd5d5), 1.0),
            RGBA::new(f(0xdcdc), f(0x3232), f(0x2f2f), 1.0),
            RGBA::new(f(0x8585), f(0x9999), f(0x0000), 1.0),
            RGBA::new(f(0xb5b5), f(0x8989), f(0x0000), 1.0),
            RGBA::new(f(0x2626), f(0x8b8b), f(0xd2d2), 1.0),
            RGBA::new(f(0xd3d3), f(0x3636), f(0x8282), 1.0),
            RGBA::new(f(0x2a2a), f(0xa1a1), f(0x9898), 1.0),
            RGBA::new(f(0x0707), f(0x3636), f(0x4242), 1.0),
            RGBA::new(f(0xfdfd), f(0xf6f6), f(0xe3e3), 1.0),
            RGBA::new(f(0xcbcb), f(0x4b4b), f(0x1616), 1.0),
            RGBA::new(f(0x9393), f(0xa1a1), f(0xa1a1), 1.0),
            RGBA::new(f(0x8383), f(0x9494), f(0x9696), 1.0),
            RGBA::new(f(0x6565), f(0x7b7b), f(0x8383), 1.0),
            RGBA::new(f(0x6c6c), f(0x7171), f(0xc4c4), 1.0),
            RGBA::new(f(0x5858), f(0x6e6e), f(0x7575), 1.0),
            RGBA::new(f(0x0000), f(0x2b2b), f(0x3636), 1.0),
        ];
    }

    fn solarized_dark() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0x0707), f(0x3636), f(0x4242), 1.0),
            RGBA::new(f(0xdcdc), f(0x3232), f(0x2f2f), 1.0),
            RGBA::new(f(0x8585), f(0x9999), f(0x0000), 1.0),
            RGBA::new(f(0xb5b5), f(0x8989), f(0x0000), 1.0),
            RGBA::new(f(0x2626), f(0x8b8b), f(0xd2d2), 1.0),
            RGBA::new(f(0xd3d3), f(0x3636), f(0x8282), 1.0),
            RGBA::new(f(0x2a2a), f(0xa1a1), f(0x9898), 1.0),
            RGBA::new(f(0xeeee), f(0xe8e8), f(0xd5d5), 1.0),
            RGBA::new(f(0x0000), f(0x2b2b), f(0x3636), 1.0),
            RGBA::new(f(0xcbcb), f(0x4b4b), f(0x1616), 1.0),
            RGBA::new(f(0x5858), f(0x6e6e), f(0x7575), 1.0),
            RGBA::new(f(0x8383), f(0x9494), f(0x9696), 1.0),
            RGBA::new(f(0x6565), f(0x7b7b), f(0x8383), 1.0),
            RGBA::new(f(0x6c6c), f(0x7171), f(0xc4c4), 1.0),
            RGBA::new(f(0x9393), f(0xa1a1), f(0xa1a1), 1.0),
            RGBA::new(f(0xfdfd), f(0xf6f6), f(0xe3e3), 1.0),
        ];
    }

    fn snazzy() -> Vec<RGBA> {
        return vec![
            RGBA::new(f(0x2828), f(0x2a2a), f(0x3636), 1.0),
            RGBA::new(f(0xffff), f(0x5c5c), f(0x5757), 1.0),
            RGBA::new(f(0x5a5a), f(0xf7f7), f(0x8e8e), 1.0),
            RGBA::new(f(0xf3f3), f(0xf9f9), f(0x9d9d), 1.0),
            RGBA::new(f(0x5757), f(0xc7c7), f(0xffff), 1.0),
            RGBA::new(f(0xffff), f(0x6a6a), f(0xc1c1), 1.0),
            RGBA::new(f(0x9a9a), f(0xeded), f(0xfefe), 1.0),
            RGBA::new(f(0xf1f1), f(0xf1f1), f(0xf0f0), 1.0),
            RGBA::new(f(0x6868), f(0x6868), f(0x6868), 1.0),
            RGBA::new(f(0xffff), f(0x5c5c), f(0x5757), 1.0),
            RGBA::new(f(0x5a5a), f(0xf7f7), f(0x8e8e), 1.0),
            RGBA::new(f(0xf3f3), f(0xf9f9), f(0x9d9d), 1.0),
            RGBA::new(f(0x5757), f(0xc7c7), f(0xffff), 1.0),
            RGBA::new(f(0xffff), f(0x6a6a), f(0xc1c1), 1.0),
            RGBA::new(f(0x9a9a), f(0xeded), f(0xfefe), 1.0),
            RGBA::new(f(0xf1f1), f(0xf1f1), f(0xf0f0), 1.0),
        ];
    }

    // =========================================================================

    pub fn colors(&self) -> Vec<RGBA> {
        return match self {
            Pallet::Tango => Self::tango(),
            Pallet::Zenburn => Self::zenburn(),
            Pallet::Linux => Self::linux(),
            Pallet::XTerm => Self::xterm(),
            Pallet::RXVT => Self::rxvt(),
            Pallet::SolarizedLight => Self::solarized_light(),
            Pallet::SolarizedDark => Self::solarized_dark(),
            Pallet::Snazzy => Self::snazzy(),
        };
    }

    pub fn all() -> Vec<Pallet> {
        return vec![
            Pallet::Tango,
            Pallet::Zenburn,
            Pallet::Linux,
            Pallet::XTerm,
            Pallet::RXVT,
            Pallet::SolarizedLight,
            Pallet::SolarizedDark,
            Pallet::Snazzy,
        ];
    }
}

