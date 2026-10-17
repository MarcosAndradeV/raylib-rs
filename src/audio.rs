use crate::core::IntoCStr;
use crate::generated::*;

pub fn init_audio_device() {
    unsafe {
        InitAudioDevice();
    }
}

pub fn close_audio_device() {
    unsafe {
        CloseAudioDevice();
    }
}

pub fn is_audio_device_ready() -> bool {
    unsafe { IsAudioDeviceReady() }
}

pub fn set_master_volume(volume: f32) {
    unsafe { SetMasterVolume(volume) }
}

pub fn get_master_volume() -> f32 {
    unsafe { GetMasterVolume() }
}

pub fn load_sound(file_name: impl IntoCStr) -> Sound {
    let c_path = file_name.into_c_string();
    unsafe { LoadSound(c_path.as_ptr()) }
}

pub fn is_sound_valid(sound: Sound) -> bool {
    unsafe { IsSoundValid(sound) }
}

pub fn unload_sound(sound: Sound) {
    unsafe { UnloadSound(sound) }
}

pub fn play_sound(sound: Sound) {
    unsafe { PlaySound(sound) }
}

pub fn stop_sound(sound: Sound) {
    unsafe { StopSound(sound) }
}

pub fn pause_sound(sound: Sound) {
    unsafe { PauseSound(sound) }
}

pub fn resume_sound(sound: Sound) {
    unsafe { ResumeSound(sound) }
}

pub fn is_sound_playing(sound: Sound) -> bool {
    unsafe { IsSoundPlaying(sound) }
}

pub fn set_sound_volume(sound: Sound, volume: f32) {
    unsafe { SetSoundVolume(sound, volume) }
}

pub fn load_music_stream(path: impl IntoCStr) -> Music {
    let c_path = path.into_c_string();
    unsafe { LoadMusicStream(c_path.as_ptr()) }
}

pub fn is_music_valid(music: Music) -> bool {
    unsafe { IsMusicValid(music) }
}

pub fn unload_music_stream(music: Music) {
    unsafe { UnloadMusicStream(music) }
}

pub fn play_music_stream(music: Music) {
    unsafe { PlayMusicStream(music) }
}

pub fn is_music_stream_playing(music: Music) -> bool {
    unsafe { IsMusicStreamPlaying(music) }
}

pub fn update_music_stream(music: Music) {
    unsafe { UpdateMusicStream(music) }
}

pub fn stop_music_stream(music: Music) {
    unsafe { StopMusicStream(music) }
}

pub fn pause_music_stream(music: Music) {
    unsafe { PauseMusicStream(music) }
}

pub fn resume_music_stream(music: Music) {
    unsafe { ResumeMusicStream(music) }
}

pub fn set_music_volume(music: Music, volume: f32) {
    unsafe { SetMusicVolume(music, volume) }
}
