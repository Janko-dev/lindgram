# Vizgram
Visualize formal grammars with L-systems

  // gekke plant
  V => [+++W][---W]YV
  W => +X[-W]Z
  X => -W[+X]Z
  Y => YZ
  Z => [-FFF][+FFF]F
  //lsys = new LSystem("VZFFF", rules, 20);


Axiom X
F --> FF
X --> F-[[X]+X]+F[+FX]-X
ø = 22.5