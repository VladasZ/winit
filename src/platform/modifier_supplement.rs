use crate::event::KeyEvent;
use crate::keyboard::Key;

/// Additional methods for the `KeyEvent` which cannot be implemented on all
/// platforms.
pub trait KeyEventExtModifierSupplement {
    /// Identical to `KeyEvent::text` but this is affected by <kbd>Ctrl</kbd>.
    ///
    /// For example, pressing <kbd>Ctrl</kbd>+<kbd>a</kbd> produces `Some("\x01")`.
    fn text_with_all_modifiers(&self) -> Option<&str>;

    /// This value ignores all modifiers including,
    /// but not limited to <kbd>Shift</kbd>, <kbd>Caps Lock</kbd>,
    /// and <kbd>Ctrl</kbd>. In most cases this means that the
    /// unicode character in the resulting string is lowercase.
    ///
    /// This is useful for key-bindings / shortcut key combinations.
    ///
    /// In case `logical_key` reports `Dead`, this will still report the
    /// key as `Character` according to the current keyboard layout. This value
    /// cannot be `Dead`.
    fn key_without_modifiers(&self) -> Key;

    /// The Latin character the system gives this physical key, lowercase, for key bindings
    /// while the layout types other letters, like Cyrillic, Greek or Hebrew.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** the virtual key code, for the letter and digit keys only.
    /// - **macOS:** the character of the key with <kbd>Cmd</kbd> held, where a non-Latin layout
    ///   gives Latin letters.
    /// - **X11 / Wayland:** the key in the first other installed layout that gives a Latin
    ///   character, unless a layout earlier in the list gives that character on another key.
    /// - **Orbital:** always `None`.
    fn latin_key(&self) -> Option<&str>;
}

impl KeyEventExtModifierSupplement for KeyEvent {
    #[inline]
    fn text_with_all_modifiers(&self) -> Option<&str> {
        self.platform_specific.text_with_all_modifiers.as_ref().map(|s| s.as_str())
    }

    #[inline]
    fn key_without_modifiers(&self) -> Key {
        self.platform_specific.key_without_modifiers.clone()
    }

    #[inline]
    fn latin_key(&self) -> Option<&str> {
        self.platform_specific.latin_key.as_deref()
    }
}
