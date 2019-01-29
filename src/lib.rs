mod tween;
mod ease;
mod lerp;
 
#[cfg(test)]
mod tests {

    use super::tween::Tween;

    struct TestA {
        value1:f32,        
        tween1:Option<Tween<f32>>
    }

    impl TestA {
        
        fn new() -> TestA {
            TestA {
               value1:0.0,
               tween1:None,
            }
        }

        fn animate_value_one(&mut self) {
            self.tween1 = Some(Tween::new(self.value1, 1.0, 0.5));
        }

        fn update(&mut self, delta:f32) {
            match &mut self.tween1 {
                Some(x) => { 
                    
                    x.tick(delta);

                    self.value1 = x.value;

                },
                None => {}
            }
        }
    }

    #[test]
    fn test_tween_struct() {
        
        let mut test = TestA::new();

        assert_eq!(test.value1, 0.0);
        
        test.animate_value_one();                

        test.update(0.5);

        assert_eq!(test.value1, -1.0);
    
    }
}
