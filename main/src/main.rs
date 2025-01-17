use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use parser::parse_lyrics;

fn main() {
    let input = "\
        [00:04.07]You're the light, you're the night
        [00:07.56]You're the color of my blood
        [00:08.81]You're the cure, you're the pain
        [00:09.56]You're the only thing I wanna touch
        [00:09.81]Never knew that it could mean so much, so much
        [00:09.81]You're the fear, I don't care
        [00:10.07]Cause I've never been so high
        [00:10.32]Follow me to the dark
        [00:10.57]Let me take you past our satellites
        [00:10.57]You can see the world you brought to life, to life
        [00:10.84]So love me like you do, love me like you do
        [00:11.06]Love me like you do, love me like you do
        [00:11.31]Touch me like you do, touch me like you do
        [00:11.56]What are you waiting for?
        [00:11.56]
        [00:11.81]Fading in, fading out
        [00:13.31]On the edge of paradise
        [00:13.56]Every inch of your skin is a holy grail I've got to find
        [00:13.56]Only you can set my heart on fire, on fire
        [00:13.81]Yeah, I'll let you set the pace
        [00:14.06]Cause I'm not thinking straight
        [00:14.32]My head spinning around I can't see clear no more
        [00:14.56]What are you waiting for?
        [00:14.56]Love me like you do, love me like you do
        [00:15.82]Love me like you do, love me like you do
        [00:16.08]Touch me like you do, touch me like you do
        [00:16.31]What are you waiting for?
        [00:16.56]Yeah, I'll let you set the pace
        [00:16.82]Cause I'm not thinking straight
        [00:17.07]My head spinning around I can't see clear no more
        [00:17.07]What are you waiting for?
        [00:18.56]Love me like you do, love me like you do
        [00:18.56]Love me like you do, love me like you do
        [00:19.07]Touch me like you do, touch me like you do
        [00:19.31]What are you waiting for?
        ";

    let start_time = Instant::now();
    let (_, lyrics) = parse_lyrics(input).unwrap();

    let mut previous_timestamp = Duration::from_secs(0);

    for lyric in lyrics {
        let wait_duration = lyric.timestamp - previous_timestamp;
        sleep(wait_duration);

        let elapsed = start_time.elapsed();
        println!("\x1B[2J\x1B[1;1H{}", lyric.text); // Clear screen and print lyric
        println!("Elapsed: {:.2}", elapsed.as_secs_f32());

        previous_timestamp = lyric.timestamp;
    }
}
