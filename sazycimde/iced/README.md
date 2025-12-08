### Goals - pragmatic
Always fast, always responsive, always re-rendering every frame.

UI can be re-programmed easily depending on the full-name context.

### Design
UI is designed around this principle:
*Your code controls an allocated portion of screen space, completely*.
The geometry of your allocated screen space is called <TODO LOJBAN>.
The code/logic/programming that controls your <GEOMETRY> is called a component, formally <TODO LOJBAN>.

There are some universal invariants that your <COMPONENT> should but are not enforced to comply with,
for example:
- Accessability <TODO LOJBAN>
- Focus outlining <TODO LOJBAN and conceptual>

There is a lot of information fed to your UI code.
It knows exactly what interaction methods are connected <TODO LOJBAN>,
and what are actively in use <TODO LOJBAN>.
<CONNECTED> and <ACTIVE> are different in that <ACTIVE> is nearly always a proper subset of <CONNECTED>,
and <ACTIVE> contains frequency desires from the user. <ACTIVE> is discussed below.

#### <ACTIVE> UX
Imagine you sit down to program at the keyboard for your laptop.
Ideally, you only need to press hjkl, and you would like to never need to press
INS or PAGE UP if your keyboard doesn't have those keys.
The current solution to this is for applications to "guess" what the most commonly used keys are,
and rely on those.
This is suboptimal UX.
