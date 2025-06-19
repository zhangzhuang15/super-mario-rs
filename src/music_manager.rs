use std::collections::HashMap;

use sdl2::{
    mixer::{Channel, Chunk, Music, open_audio},
    sys::mixer::MIX_DEFAULT_FORMAT,
};

pub(crate) struct MusicManager {
    /// Save long music, such as background music
    music_table: HashMap<&'static str, Music<'static>>,
    /// Save short music, such as Mario jumping music
    chunk_music_table: HashMap<&'static str, Chunk>,
    pub volume: i32,
    current_music_name: Option<&'static str>,
    current_chunk_name: Option<&'static str>,
    music_is_stopped: bool,
}

impl MusicManager {
    pub fn new() -> Self {
        open_audio(44100, MIX_DEFAULT_FORMAT as u16, 2, 2048).expect("fail to open audio");

        let mut music_table: HashMap<&'static str, Music> = HashMap::new();

        let wav_names = vec![
            "overworld",
            "overworld-fast",
            "underground",
            "underground-fast",
            "underwater",
            "underwater-fast",
            "castle",
            "castle-fast",
            "lowtime",
            "starmusic",
            "starmusic-fast",
            "scorering",
        ];
        wav_names.into_iter().for_each(|wav_name| {
            music_table.insert(
                wav_name,
                Music::from_file(format!("files/sounds/{}.wav", wav_name))
                    .expect(format!("fail to load {}.wav", wav_name).as_str()),
            );
        });

        let mut chunk_music_table: HashMap<&'static str, Chunk> = HashMap::new();
        let chunk_music_names = vec![
            "coin",
            "blockbreak",
            "blockhit",
            "boom",
            "bowserfall",
            "bridgebreak",
            "bulletbill",
            "death",
            "fire",
            "fireball",
            "gameover",
            "intermission",
            "jump",
            "jumpbig",
            "levelend",
            "lowtime",
            "mushroomappear",
            "mushroomeat",
            "oneup",
            "pause",
            "shrink",
            "rainboom",
            "shot",
            "shrink",
            "stomp",
            "swim",
            "vine",
            "castleend",
            "princessmusic",
        ];
        chunk_music_names.into_iter().for_each(|chunk_music_name| {
            chunk_music_table.insert(
                chunk_music_name,
                Chunk::from_file(format!("files/sounds/{}.wav", chunk_music_name))
                    .expect(format!("fail to load chunk music {}.wav", chunk_music_name).as_str()),
            );
        });

        let volume = 100;
        Music::set_volume(volume);

        MusicManager {
            volume,
            chunk_music_table,
            music_table,
            current_chunk_name: None,
            current_music_name: None,
            music_is_stopped: true,
        }
    }

    pub fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
        Music::set_volume(volume);
    }

    pub fn stop_current_music(&mut self) {
        if self.music_is_stopped {
            return;
        }

        Music::halt();
        self.music_is_stopped = true;
    }

    /// If current music is paused, we resume it;
    /// If current music is running, we pause it;
    pub fn pause_or_recover_current_music(&mut self) {
        if Music::is_paused() {
            Music::resume();
            self.music_is_stopped = false;
            return;
        }

        Music::pause();
        self.music_is_stopped = true;
    }

    pub fn play_current_music(&mut self) {
        if !self.music_is_stopped {
            return;
        }

        match self.current_music_name {
            None => {
                println!("current music is none! refuse to play music!");
            }
            Some(music_name) => {
                let music = self.music_table.get_mut(music_name);
                if music.is_none() {
                    println!("cannot find music {}.wav", music_name);
                } else {
                    let music = music.unwrap();
                    music.play(-1);
                    self.music_is_stopped = false;
                }
            }
        }
    }

    pub fn play_specific_music(&mut self, music_name: &'static str) {
        if !self.music_table.contains_key(music_name) {
            self.stop_current_music();
            return;
        }

        let music = self.music_table.get_mut(music_name).unwrap();
        music.play(-1);

        self.music_is_stopped = false;
        self.current_music_name = music_name.into();
    }

    pub fn play_chunk_music(&mut self, chunk_music_name: &'static str) {
        if !self.chunk_music_table.contains_key(chunk_music_name) {
            println!("cannot find {}.wav, refuse to play it", chunk_music_name);
            return;
        }

        let chunk_music = self.chunk_music_table.get_mut(chunk_music_name).unwrap();
        chunk_music.set_volume(self.volume);

        Channel::all().play(&chunk_music, 0);
        self.current_chunk_name = chunk_music_name.into();
    }
}
