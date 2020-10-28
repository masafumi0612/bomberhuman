/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
//    pub rotate_left: bool,
//    pub rotate_right: bool,
//    pub boost: bool,
    pub put_bomb: bool
}
