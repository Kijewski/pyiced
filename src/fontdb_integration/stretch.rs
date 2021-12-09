crate::wrap_rust_enum!(
    /// A CSS `font-stretch <https://developer.mozilla.org/en-US/docs/Web/CSS/font-stretch>`_.
    ///
    /// See also
    /// --------
    /// `fontdb::Stretch <https://docs.rs/fontdb/0.7.0/fontdb/enum.Stretch.html>`_
    "FontStretch" -> WrappedFontStretch(fontdb::Stretch) {
        /// 50% width
        ULTRACONDENSED -> UltraCondensed,
        /// 62.5% width
        EXTRACONDENSED -> ExtraCondensed,
        /// 75% width
        CONDENSED -> Condensed,
        /// 87.5% width
        SEMICONDENSED -> SemiCondensed,
        /// 100% width
        NORMAL -> Normal,
        /// 112.5% width
        SEMIEXPANDED -> SemiExpanded,
        /// 125% width
        EXPANDED -> Expanded,
        /// 150% width
        EXTRAEXPANDED -> ExtraExpanded,
        /// 200% width
        ULTRAEXPANDED -> UltraExpanded,
    }
);
