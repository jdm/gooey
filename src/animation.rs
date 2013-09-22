use std::comm;
use std::rt::io::timer::Timer;
use std::task;

type AnimationAdvancer = @fn() -> Option<u64>;

struct Animation {
    port: comm::Port<()>,
    advance: AnimationAdvancer
}

pub struct AnimationManager {
    animations: ~[Animation],
}

impl AnimationManager {
    pub fn new() -> AnimationManager {
        AnimationManager {
            animations: ~[]
        }
    }

    fn spawn_timer(delay: u64, chan: Chan<()>) {
        do spawn {
            let mut timer = Timer::new().unwrap();
            timer.sleep(delay);
            chan.send(());
        }
    }

    pub fn add(&mut self, advance: AnimationAdvancer, initial_delay: u64) {
        let (port, chan) = comm::stream();
        AnimationManager::spawn_timer(initial_delay, chan);
        let anim = Animation {
            advance: advance,
            port: port,
        };
        self.animations.push(anim);
    }

    pub fn run(&mut self) {
        let mut removed = ~[];
        for (i, anim) in self.animations.mut_iter().enumerate() {
            loop {
                if !anim.port.peek() {
                    break
                }
                anim.port.recv();
                let delay = (anim.advance)();
                match delay {
                    Some(delay) => {
                        let (port, chan) = comm::stream();
                        anim.port = port;
                        AnimationManager::spawn_timer(delay, chan);
                    }
                    None => removed.push(i)
                }
            }
        }
        for &index in removed.rev_iter() {
            self.animations.remove(index);
        }
        task::deschedule(); //HACK: peek doesn't yield
    }
}
