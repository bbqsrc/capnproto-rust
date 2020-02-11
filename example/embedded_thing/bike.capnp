@0x9b81cef8379807ad;

struct Bike {
  id @0 :UInt32;
  owner @1 :Text;
  model @2 :Text;
  color @3 :Color;
  
  enum Color {
    blue @0;
    red @1;
    rusted @2;
  }
}
