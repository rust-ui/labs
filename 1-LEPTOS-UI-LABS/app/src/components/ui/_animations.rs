#[allow(non_snake_case)]
pub struct Animations {
    // Default Tailwind CSS animations.
    pub PULSE: &'static str,
    pub PING: &'static str,
    // Custom animations.
    pub AFTER_BEAM_BORDER: &'static str,
    pub BACKGROUND_RETRO_GRID: &'static str,
    pub BLURRY_BLOB: &'static str,
    pub GRADIENT: &'static str,
    pub MARQUEE: &'static str,
    pub METEOR_EFFECT: &'static str,
    pub ORBITING_CIRCLE_ITEM: &'static str,
    pub RADAR_SPIN: &'static str,
    pub RADAR_MINI: &'static str,
    pub TOAST_TRACKER: &'static str,
    pub TYPING_EFFECT: &'static str,
    pub TEXT_SWIPING: &'static str,
    pub SPOTLIGHT_EFFECT: &'static str,
}

pub const ANIMATIONS: Animations = Animations {
    // Default Tailwind CSS animations.
    PULSE: "animate-pulse",
    PING: "animate-ping",
    // Custom animations.
    AFTER_BEAM_BORDER: "after:animate-afterBeamBorder",
    BACKGROUND_RETRO_GRID: "animate-backgroundRetroGrid",
    BLURRY_BLOB: "animate-blurryBlob",
    GRADIENT: "animate-gradient",
    MARQUEE: "animate-marquee",
    METEOR_EFFECT: "animate-meteorEffect",
    ORBITING_CIRCLE_ITEM: "animate-orbitingCircleItem",
    RADAR_SPIN: "animate-radarSpin",
    RADAR_MINI: "animate-radarMini",
    TOAST_TRACKER: "animate-toastTracker",
    TYPING_EFFECT: "animate-typingEffect",
    TEXT_SWIPING: "animate-textSwiping",
    SPOTLIGHT_EFFECT: "animate-spotlightEffect",
};
