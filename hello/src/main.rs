#[derive(Debug)]
struct CubeSat {
  id: u64,
  mailbox: Mailbox,
}

struct GroundStation;

impl GroundStation {
  fn send(&self, to: &mut CubeSat, msg: Message) {
    to.mailbox.messages.push(msg);
  }
}

impl CubeSat {
  fn recv(&mut self) -> Option<Message> {
    self.mailbox.messages.pop()
  }
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

type Message = String;

fn main() {

}