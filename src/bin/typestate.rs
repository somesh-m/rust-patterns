/* Typestate pattern is used to ensure compile time guarantee that only valid operations can be performed. Instead of tracking state with runtime flags or if/else check, compile time guarantee can be more robust
 */

/*
Lets assume we have to design a state machine for a media player
                       Stop
     -----------------------------------------
    |                   |                    |
    |                   |                    |
    ↓                   ↑                    ↑
Stopped ----------- Buffering ----------- Playing ----------- Paused
           Play                                      Pause
 We can design using enums as states and then check for the action which is being passed on, eventually accepting or rejcting it based on current state. But there is a better way to do it.
*/

use std::marker::PhantomData;

struct Stopped;
struct Buffering;
struct Playing;
struct Paused;

struct Media<State> {
    track_name: String,
    _state: PhantomData<State>,
}

impl Media<Stopped> {
    fn new(track_name: String) -> Self {
        Self {
            track_name,
            _state: PhantomData,
        }
    }

    fn play(self) -> Media<Playing> {
        println!("Buffering track {}", self.track_name);

        let media = Media::<Buffering> {
            track_name: self.track_name,
            _state: PhantomData,
        };

        media.buffer()
    }
}

impl Media<Buffering> {
    fn buffer(self) -> Media<Playing> {
        for i in 1..=5 {
            std::thread::sleep(std::time::Duration::from_secs(1));

            println!("Buffering: {i}/5");
        }

        Media {
            track_name: self.track_name,
            _state: PhantomData,
        }
    }
    fn stop(self) -> Media<Stopped> {
        println!("Stopping the song: {}", self.track_name);

        Media {
            track_name: self.track_name,
            _state: PhantomData,
        }
    }
}

impl Media<Playing> {
    fn stop(self) -> Media<Stopped> {
        println!("Stopping the song: {}", self.track_name);

        Media {
            track_name: self.track_name,
            _state: PhantomData,
        }
    }

    fn pause(self) -> Media<Paused> {
        println!("Pausing the song: {}", self.track_name);

        Media {
            track_name: self.track_name,
            _state: PhantomData,
        }
    }
}

fn main() {
    let media = Media::<Stopped>::new("sample_track".into());
    let media = media.play();
    let media = media.pause();
}
