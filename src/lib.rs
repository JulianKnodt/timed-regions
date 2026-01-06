use std::time::{Duration, Instant};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    Active,
    #[default]
    Inactive,
}

#[derive(Debug, Clone, Copy)]
pub struct TimeCounter {
    pub duration: Duration,
    pub count: u32,
    pub recent: Instant,
    state: State,
}

impl Default for TimeCounter {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            count: 0,
            recent: Instant::now(),
            state: State::Inactive,
        }
    }
}

impl TimeCounter {
    pub fn start(&mut self) {
        self.recent = Instant::now();
        self.count += 1;
        assert_eq!(self.state, State::Inactive);
        self.state = State::Active;
    }
    pub fn stop(&mut self) {
        self.duration += self.recent.elapsed();
        assert_eq!(self.state, State::Active);
        self.state = State::Inactive;
    }
}

#[macro_export]
macro_rules! TimerStruct {

  (struct $s_name: ident {$(
    $name: ident,
  )+} $(,)?) => {

    #[derive(Debug, Clone, Copy, Default)]
    pub struct $s_name {
      $(
        pub $name: $crate::TimeCounter,
      )+
    }

    impl $s_name {
      pub fn to_json(&self, dst: impl std::io::Write) -> std::io::Result<()> {
        use std::io::Write;
        let mut out = std::io::BufWriter::new(dst);
        writeln!(out, "{{")?;
        let mut left = 0$(+ {
          #[allow(unused)]
          let $name: ();
          1
        })+;
        $(
          let n = stringify!($name);
          let dur = self.$name.duration.as_secs_f32() / self.$name.count as f32;
          left -= 1;
          let comma = if left == 0 {
            ""
          } else {
            ","
          };
          writeln!(out, "  \"{n}\": {dur}{comma}")?;
        )+
        writeln!(out, "}}")
      }

      #[allow(unused)]
      pub fn print(&self) {
        $(
          let dur = self.$name.duration.as_secs_f32() / self.$name.count as f32;
          println!("{}: {dur}", stringify!($name));
        )+
      }
    }
  };
}

#[test]
fn test() {
    TimerStruct!(
      struct Timey {
        a,
        b,
        c,
      },
    );

    let mut t = Timey::default();
    t.a.start();
    println!("tmp");
    t.a.stop();

    let mut buf = vec![];
    let _ = t.to_json(&mut buf);
    let string = String::from_utf8(buf).unwrap();
    println!("{string}");
}
