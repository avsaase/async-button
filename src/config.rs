use super::Duration;

/// Various [`Button`](super::Button) parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ButtonConfig {
    /// How much time the button should be pressed to in order to count it as a click.
    pub debounce: Duration,
    /// How much time the button between consecutive clicks to count as a double click.
    pub double_click: Duration,
    /// How much time the button is held before a long press is detected.
    pub long_press: Duration,
    /// Button direction.
    pub mode: Mode,
}

impl ButtonConfig {
    /// Returns new [ButtonConfig].
    ///
    /// As a general rule, `debounce` time is less then `double_click` time and `long_press` time is larger than both.
    pub fn new(
        debounce: Duration,
        double_click: Duration,
        long_press: Duration,
        mode: Mode,
    ) -> Self {
        Self {
            debounce,
            double_click,
            long_press,
            mode,
        }
    }
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            debounce: Duration::from_millis(10),
            double_click: Duration::from_millis(350),
            long_press: Duration::from_millis(1000),
            mode: Mode::default(),
        }
    }
}

/// Button direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Active 0.
    #[default]
    PullUp,
    /// Active 1.
    PullDown,
}

impl Mode {
    /// Is button activated by logic zero?
    pub const fn is_pullup(&self) -> bool {
        matches!(self, Mode::PullUp)
    }

    /// Is button activated by logic one?
    pub const fn is_pulldown(&self) -> bool {
        !self.is_pullup()
    }
}
