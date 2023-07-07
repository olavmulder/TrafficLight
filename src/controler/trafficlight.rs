use hal::gpio::PushPullOutput;
// Alias for our HAL crate
use rp2040_hal as hal;
// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use embedded_hal::digital::v2::ToggleableOutputPin;
// Our interrupt macro
// Some short-cuts to useful types
use rp2040_hal::gpio;
// The GPIO interrupt type we're going to generate
use hal::pac::interrupt;
// Some short-cuts to useful types
use core::cell::RefCell;
use critical_section::Mutex;
use rp2040_hal::gpio::Interrupt::EdgeLow;

#[derive(PartialEq)] pub enum State {
   Red = 0,
   Yellow ,
   Green,
}

type ButtonPin = gpio::Pin<gpio::bank0::Gpio15, gpio::FloatingInput>;
type LedPin = gpio::Pin<gpio::bank0::Gpio19, gpio::PushPullOutput>;
type LedAndButton = (ButtonPin, LedPin);
static GLOBAL_PINS: Mutex<RefCell<Option<LedAndButton>>> = Mutex::new(RefCell::new(None));

#[interrupt]

unsafe fn IO_IRQ_BANK0() {
    // The `#[interrupt]` attribute covertly converts this to `&'static mut Option<LedAndButton>`
    static mut LED_AND_BUTTON: Option<LedAndButton> = None;

    // This is one-time lazy initialisation. We steal the variables given to us
    // via `GLOBAL_PINS`.
    if LED_AND_BUTTON.is_none() {
        critical_section::with(|cs| {
            *LED_AND_BUTTON = GLOBAL_PINS.borrow(cs).take();
        });
    }

    // Need to check if our Option<LedAndButtonPins> contains our pins
    if let Some(gpios) = LED_AND_BUTTON {
        // borrow led and button by *destructuring* the tuple
        // these will be of type `&mut LedPin` and `&mut ButtonPin`, so we don't have
        // to move them back into the static after we use them
        let (button, led) = gpios;
        // Check if the interrupt source is from the pushbutton going from high-to-low.
        // Note: this will always be true in this example, as that is the only enabled GPIO interrupt source
        if button.interrupt_status(EdgeLow) {
            // toggle can't fail, but the embedded-hal traits always allow for it
            // we can discard the return value by assigning it to an unnamed variable
            let _ = led.toggle();

            // Our interrupt doesn't clear itself.
            // Do that now so we don't immediately jump back to this interrupt handler.
            button.clear_interrupt(EdgeLow);
        }
    }
}
struct Gpio
{  
   red   :  gpio::Pin<gpio::bank0::Gpio21, PushPullOutput>,
   green :  gpio::Pin<gpio::bank0::Gpio20, PushPullOutput>,
}

impl Gpio
{
   pub fn new(pins: hal::gpio::Pins) -> Self
   {
      let button_pin = pins.gpio15.into_floating_input();
      let led_pin = pins.gpio19.into_push_pull_output();
     
      button_pin.set_interrupt_enabled(EdgeLow, true);
      critical_section::with(|cs| {
         GLOBAL_PINS.borrow(cs).replace(Some((button_pin, led_pin)));
      });
     
 
      Gpio { 
         red   : pins.gpio21.into_push_pull_output(),
         green : pins.gpio20.into_push_pull_output(),
      }
   }

   pub fn ToggleRedLed(&mut self)
   {
      self.red.toggle().unwrap();
   }

}

pub struct TrafficLight
{
   pub state: State,
   led : Gpio
}


impl TrafficLight
{
   pub fn  new(pins: hal::gpio::Pins) -> Self
   {
      TrafficLight
      {
         state: State::Red,
         led : Gpio::new(pins)
      }
   }

   pub fn SetState(&mut self, st: State)
   {
      self.state = st;
      Gpio::ToggleRedLed(&mut self.led);
   }
}
