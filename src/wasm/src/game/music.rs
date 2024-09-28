pub mod music {
    use crate::game::{Audio, Sound};

    #[derive(Clone)]
    pub struct Music {
        pub audio: Audio,
        pub sound: Sound,
    }
    impl Music {
        pub fn new(audio: Audio, sound: Sound) -> Self {
            Music {
                audio: audio,
                sound: sound,
            }
        }
        pub fn play_brake_sound(self) -> Self{
            if let Err(err) = self.audio.play_sound(&self.sound) {
                log!("Error playing jump sound {:#?}", err);
            }
            self
        }
    }
}