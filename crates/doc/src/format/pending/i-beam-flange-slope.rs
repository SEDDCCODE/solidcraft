//! An I beam profile can save `flange_slope` (an angle; its flanges' inner faces taper, thicker
//! at the web, as on taper flange beams) and `toe_radius` (the round at each flange's inner toe).
//! An older design has neither: its I beams have parallel flanges and sharp toes, which is what
//! a beam without them still builds. The last release warns about the newer format number; it
//! can't read a frame member feature whose I beam has one of these keys, which keeps its place
//! on the timeline unevaluated and is saved back as it came.
