//! Music module for handling audio playback in the game.
//! Provides functionality for playing sound effects and managing audio resources.

pub mod music {
    use crate::game::{Audio, Sound};

    /// Music struct manages audio playback for game sounds.
    /// Contains audio interface and sound data for sound effect playback.
    #[derive(Clone)]
    pub struct Music {
        pub audio: Audio,
        pub sound: Sound,
    }
    impl Music {
        /// Creates a new Music instance with the provided audio interface and sound data.
        ///
        /// # Arguments
        /// * `audio` - Audio interface for sound playback
        /// * `sound` - Sound data to be played
        ///
        /// # Returns
        /// A new Music instance
        pub fn new(audio: Audio, sound: Sound) -> Self {
            Music {
                audio: audio,
                sound: sound,
            }
        }
        /// Plays the brake sound effect using the audio interface.
        /// Logs an error message if sound playback fails.
        ///
        /// # Arguments
        /// * `self` - Music instance (consumed and returned)
        ///
        /// # Returns
        /// The same Music instance for method chaining
        pub fn play_brake_sound(self) -> Self {
            if let Err(err) = self.audio.play_sound(&self.sound) {
                log!("Error playing jump sound {:#?}", err);
            }
            self
        }
    }
}
