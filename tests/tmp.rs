use timed_regions::TimerStruct;

#[test]
pub fn test_construct() {
    TimerStruct!(
      struct Test {
        a, b, c,
      }
    );
}
