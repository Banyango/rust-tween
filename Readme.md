# Interpolation Experiment

A library for interpolating values according to different functions. 

Note: Not intended for production, I'm using this to learn Rust better. 

## Usage 

Create a type tween. This will set the tween to alive, and then increment the tween by supplying a delta_time tick.

Sample usage

```
struct A {
    tween:Tween<f32>
}

impl A {

    fn tween_me(&mut self) {
        tween = Tween::new(0.0, 500.0, 2.0);
        tween.set_ease(EaseType::SineIn);        
    }

    fn update_tick(&mut self, delta_time:f32) {
        tween.tick(delta_time);

        println!("interpolated value is = {}",tween.value);
    }

}
```

###todo 
- Add support for elastic, bounce, and back interpolation types.
- Figure out a nicer way to have the tween update a reference value...