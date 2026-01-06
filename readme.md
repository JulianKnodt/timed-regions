# Timed Regions

Simple macro for timing regions in rust.

```
TimerStruct! {
  struct MyTimer {
    a,
    b,
  }

  let t = MyTimer::default();
  t.a.start();
  t.b.start();
  t.a.stop();
  t.b.stop();
}
```
