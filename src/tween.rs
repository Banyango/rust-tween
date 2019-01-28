use super::ease::EaseType;
use super::lerp::Lerp;

//
// TERMS OF USE - EASING EQUATIONS
//
// Open source under the BSD License.
//
// Copyright © 2001 Robert Penner
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
// - Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
// - Neither the name of the author nor the names of contributors may be used to endorse
// or promote products derived from this software without specific prior written permission.
// - THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
// EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

pub struct Tween<T> where T : Lerp {    
    pub value:T,
    end_value:T,
    time:f32,
    duration:f32,    
    pub alive:bool, 
    ease: EaseType
}

impl<T> Tween<T> where T : Lerp {

    pub fn new(value:T, end:T, duration:f32) -> Tween<T> {
        Tween {
            value:value,
            end_value:end,
            time:0.0,
            duration:duration,            
            alive:true,
            ease:EaseType::QuadraticIn,
        }
    } 

    pub fn tick(&mut self, delta_time: f32) {
            
        if self.time >= 1.0 {
            self.alive = false;
        } else {               
            self.time+=delta_time;

            let value = self.evaluate(&self.ease, self.time, self.duration);

            let amount = self.value.lerp(&self.end_value, value);

            self.value = amount;
        }
    }
    /*
     *  Returns a value between 0 and 1 inclusive based on elapsed time 
     */
    fn evaluate(&self, ease: &EaseType, current_time: f32, duration: f32) -> f32 {

        let mut time = current_time;

        match ease {
            EaseType::QuadraticIn => {
                time /= duration;
                return -time * time;
            },
            EaseType::QuadraticOut => {
                time /= duration;
                return -time * (time - 2.0);
            },
            EaseType::QuadraticInOut => {
                time /= duration;
                if time * 0.5 < 1.0 {
                    return 0.5 * time * time;
                }
                return -0.5 * ((--time) * (time - 2.0) - 1.0);
            }
            EaseType::CubicIn => {
                time /= duration;
                return time * time * time
            },
            EaseType::CubicOut => {
                time = time / duration - 1.0;
                return time * time * time + 1.0
            },
            EaseType::CubicInOut => {
                time /= duration * 0.5;
                if time < 1.0 { 
                    return 0.5 * time * time * time;
                }                
                time -= 2.0;                
                return 0.5 * (time * time * time + 2.0);
            },
            EaseType::QuarticIn => {
                time /= duration;
                return time * time * time * time;
            },
            EaseType::QuarticOut => {
                time = time / duration - 1.0;
                return -(time * time * time * time - 1.0);
            },
            EaseType::QuarticInOut => {
                time /= duration;
                if (time * 0.5) < 1.0 {
                    return 0.5 * time * time * time * time;
                } 
                time -= 2.0;
                return -0.5 * (time * time * time * time - 2.0);
            },
            EaseType::QuinticIn => {
                time /= duration;
                return time * time * time * time * time;
            },
            EaseType::QuinticOut => {
                time = time / duration - 1.0;
                return time * time * time * time * time + 1.0
            },
            EaseType::QuinticInOut => {
                time /= duration;
                if (time * 0.5) < 1.0 {
                    return 0.5 * time * time * time * time * time;
                }
                time -= 2.0;
                return 0.5 * (time * time * time * time * time + 2.0);
            },
            EaseType::SineIn => {
                let c : f32 = time / duration * (std::f32::consts::PI/2.0);
                return -c.cos() + 1.0;
            },
            EaseType::SineOut => {
                let c : f32 = time / duration * (std::f32::consts::PI/2.0);
                return c.sin();
            },
            EaseType::SineInOut => {
                let c : f32 = std::f32::consts::PI * time / duration;
                return -0.5 * (c.cos() - 1.0);
            },
            EaseType::CircularIn => {
                time /= duration;
                let c : f32 = 1.0 - time * time;
                return -(c.sqrt() - 1.0);
            },
            EaseType::CircularOut => {
                time = time / duration - 1.0;
                let c : f32 = 1.0 - time * time;
                return c.sqrt();
            },
            EaseType::CircularInOut => {
                time /= duration;
                if (time * 0.5) < 1.0 {
                    let c: f32 = 1.0 - time * time;
                    return -0.5 * (c.sqrt() - 1.0);
                }

                time -= 2.0;
                let c : f32 = 1.0 - time * time;
                return 0.5 * (c.sqrt() + 1.0);
            },
            EaseType::ExponentialIn => {
                if time == 0.0 {
                     return 0.0
                } 
                let c :f32 = 10.0 * (time / duration - 1.0);
                return (2.0 as f32).powf(c);
            },
            EaseType::ExponentialOut => {
                if time == duration {
                     return 1.0;
                }
                return (-2.0 as f32).powf(-10.0 * time / duration) + 1.0;
            },
            EaseType::ExponentialInOut => {
                match time {
                    p if p == 0.0 => return 0.0,
                    p if p == duration => return 1.0,
                    p if p / duration * 0.5 < 1.0 => return 0.5 * (2.0 as f32).powf(10.0 * (time/duration - 1.0)),                    
                    _ => return 0.5 * ((-2.0 as f32).powf(-10.0 * --time) + 2.0)       
                }                        
            },
            _ => return 0.0
            // todo finish these lerp equations.
            // EaseType::ElasticIn,
            // EaseType::ElasticOut,
            // EaseType::ElasticInOut,

            // EaseType::BackIn,
            // EaseType::BackOut,
            // EaseType::BackInOut,

            // EaseType::BounceIn,
            // EaseType::BounceOut,
            // EaseType::BounceInOut,
        }

    }   

    pub fn set_ease(&mut self, ease: EaseType) {
        self.ease = ease;
    }   

    //pub fn on_complete(on_complete: )

    
    
}

#[cfg(test)]
mod tween_tests {

    use super::Tween;
    use super::EaseType;

    #[test]
    fn test_evaluate_quadratic_in() {
        let value = 0.0;
        
        let mut tween = Tween::new(value, 1.0, 0.5);
        
        tween.tick(0.5);

        assert_eq!(tween.value, -1.0);
    }
    #[test]
    fn test_evaluate_quadratic_out() {
        let value = 0.0;
        
        let mut tween = Tween::new(value, 1.0, 0.5);
        
        tween.ease = EaseType::QuadraticOut;

        tween.tick(0.5);

        assert_eq!(tween.value, 1.0);
    }
}
