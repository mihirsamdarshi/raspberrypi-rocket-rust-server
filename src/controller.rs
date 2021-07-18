use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::gpio::{Gpio, IoPin, Mode};

/// ENA  --[==()==]-- VCC1
/// IN1  --[======]-- IN4
/// OUT1 --[==L===]-- OUT4
/// GND1 --[==2===]-- GND4
/// GND2 --[==9===]-- GND3
/// OUT2 --[==3===]-- OUT3
/// IN2  --[==D===]-- IN3
/// VCC2 --[======]-- ENB
pub struct L293D {
  ldirf: Option<IoPin>,
  ldirb: Option<IoPin>,
  rdirf: Option<IoPin>,
  rdirb: Option<IoPin>,
  lpwm: Option<Pwm>,
  rpwm: Option<Pwm>
}

impl L293D {
  pub fn new() -> Self {
    return L293D {
      ldirf: None,
      ldirb: None,
      rdirf: None,
      rdirb: None,
      lpwm: None,
      rpwm: None
    };
  }

  /**
   * Sets the speed control gpio pin of the left hand
   * side of the chip.
   */
  pub fn with_lpwm_speed(mut self) -> Self {
    self.lpwm = Some(Pwm::with_frequency(Channel::Pwm0, 0.0, 0.5, Polarity::Normal, false).unwrap());
    return self;
  }

  /**
   * Sets the direction control pins of the left hand
   * side of the chip
   */
  pub fn with_ldirection_pins(mut self, forward_pin: u8, backward_pin: u8) -> Self {
    let controller = Gpio::new().unwrap();
    let fpin = controller.get(forward_pin).unwrap();
    let bpin = controller.get(backward_pin).unwrap();

    self.ldirf = Some(fpin.into_io(Mode::Input));
    self.ldirb = Some(bpin.into_io(Mode::Input));
    return self;
  }

  /**
   * Sets the speed control pin of the right hand
   * side of the chip
   */
  pub fn with_rpwm_speed(mut self) -> Self {
    self.rpwm = Some(Pwm::with_frequency(Channel::Pwm1, 0.0, 0.5, Polarity::Normal, false).unwrap());
    return self;
  }

  /**
   * Sets the direction control pins of the right hand
   * side of the chip.
   */
  pub fn with_rdirection_pins(mut self, forward_pin: u8, backward_pin: u8) -> Self {
    let controller = Gpio::new().unwrap();
    let fpin = controller.get(forward_pin).unwrap();
    let bpin = controller.get(backward_pin).unwrap();

    self.rdirf = Some(fpin.into_io(Mode::Input));
    self.rdirb = Some(bpin.into_io(Mode::Input));
    return self;
  }

  pub fn activate_lpwm_speed(self) {
    self.lpwm.unwrap().enable().unwrap();
  }

  pub fn set_lspeed(self, hertz: f64) {
    self.lpwm.unwrap().set_frequency(hertz, 0.5);
  }

  /**
   * Drives the left hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn lforward(self) {
    self.ldirb.unwrap().set_low();
    self.ldirf.unwrap().set_high();
  }

  /**
   * Drives the left hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn lbackward(self) {
    self.ldirb.unwrap().set_high();
    self.ldirf.unwrap().set_low();
  }

  pub fn activate_rpwm_speed(&self) {
    self.lpwm.as_ref().unwrap().enable().unwrap();
  }

  pub fn set_rspeed(&self, hertz: f64) {
    self.rpwm.as_ref().unwrap().set_frequency(hertz, 0.5);
  }

  /**
   * Drives the right hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn rforward(&mut self) {
    self.rdirb.as_mut().unwrap().set_low();
    self.rdirf.as_mut().unwrap().set_high();
  }

  /**
   * Drives the right hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn rbackward(&mut self) {
    self.rdirb.as_mut().unwrap().set_high();
    self.rdirf.as_mut().unwrap().set_low();
  }

}