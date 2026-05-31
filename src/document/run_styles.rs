
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum RunSemanticStyle {
  #[default]
  Plain,
  Cite,
  Emphasis,
  Underline,
  Condensed,
  Ultracondensed,
  Custom(u8),
}

impl RunSemanticStyle {
  #[must_use]
  pub const fn slot(self) -> u64 {
    match self {
      Self::Plain => 0,
      Self::Cite => 1,
      Self::Emphasis => 2,
      Self::Underline => 3,
      Self::Condensed => 4,
      Self::Ultracondensed => 5,
      Self::Custom(slot) => 128 + slot as u64,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum HighlightStyle {
  Spoken,
  Insert,
  Alternative,
  Custom(u8),
}

impl HighlightStyle {
  #[must_use]
  pub const fn slot(self) -> u64 {
    match self {
      Self::Spoken => 1,
      Self::Insert => 2,
      Self::Alternative => 3,
      Self::Custom(slot) => 128 + slot as u64,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunStyle {
  Plain,
  Cite,
  Underline,
  Emphasis,
  Condensed,
  Ultracondensed,
  HighlightSpoken,
  HighlightInsert,
  HighlightAlternative,
}

#[hotpath::measure_all]
impl From<RunStyle> for RunStyles {
  fn from(style: RunStyle) -> Self {
    let mut styles = Self::default();
    styles.apply(style);
    styles
  }
}

#[hotpath::measure_all]
impl RunStyles {
  pub const fn apply(&mut self, style: RunStyle) {
    match style {
      RunStyle::Plain => self.semantic = RunSemanticStyle::Plain,
      RunStyle::Cite => self.semantic = RunSemanticStyle::Cite,
      RunStyle::Underline => self.semantic = RunSemanticStyle::Underline,
      RunStyle::Emphasis => self.semantic = RunSemanticStyle::Emphasis,
      RunStyle::Condensed => self.semantic = RunSemanticStyle::Condensed,
      RunStyle::Ultracondensed => self.semantic = RunSemanticStyle::Ultracondensed,
      RunStyle::HighlightSpoken => self.highlight = Some(HighlightStyle::Spoken),
      RunStyle::HighlightInsert => self.highlight = Some(HighlightStyle::Insert),
      RunStyle::HighlightAlternative => self.highlight = Some(HighlightStyle::Alternative),
    }
  }

  #[must_use]
  pub const fn with(mut self, style: RunStyle) -> Self {
    self.apply(style);
    self
  }

  #[must_use]
  pub const fn with_direct_underline(mut self) -> Self {
    self.direct_underline = true;
    self
  }

  #[must_use]
  pub const fn with_strikethrough(mut self) -> Self {
    self.strikethrough = true;
    self
  }
}

// -- Theme ----------------------------------------------------------------
