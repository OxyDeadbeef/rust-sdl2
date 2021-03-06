use libc::{c_int, c_char, c_void, int32_t, int16_t, int8_t, uint8_t};

pub type SDL_bool = c_int;

pub type SDL_Joystick = c_void;

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct SDL_JoystickGUID {
    pub data: [uint8_t; 16],
}

extern "C" {
    pub fn SDL_NumJoysticks() -> c_int;
    pub fn SDL_JoystickNameForIndex(device_index: c_int) -> *const c_char;
    pub fn SDL_JoystickOpen(device_index: c_int) -> *mut SDL_Joystick;
    pub fn SDL_JoystickName(joystick: *mut SDL_Joystick) -> *const c_char;
    pub fn SDL_JoystickGetDeviceGUID(device_index: c_int) ->
              SDL_JoystickGUID;
    pub fn SDL_JoystickGetGUID(joystick: *mut SDL_Joystick) ->
              SDL_JoystickGUID;
    pub fn SDL_JoystickGetGUIDString(guid: SDL_JoystickGUID,
                                           pszGUID: *const c_char, cbGUID: c_int);
    pub fn SDL_JoystickGetGUIDFromString(pchGUID: *const c_char) ->
              SDL_JoystickGUID;
    pub fn SDL_JoystickGetAttached(joystick: *mut SDL_Joystick) -> SDL_bool;
    pub fn SDL_JoystickInstanceID(joystick: *mut SDL_Joystick) -> int32_t;
    pub fn SDL_JoystickNumAxes(joystick: *mut SDL_Joystick) -> c_int;
    pub fn SDL_JoystickNumBalls(joystick: *mut SDL_Joystick) -> c_int;
    pub fn SDL_JoystickNumHats(joystick: *mut SDL_Joystick) -> c_int;
    pub fn SDL_JoystickNumButtons(joystick: *mut SDL_Joystick) -> c_int;
    pub fn SDL_JoystickUpdate();
    pub fn SDL_JoystickEventState(state: c_int) -> c_int;
    pub fn SDL_JoystickGetAxis(joystick: *mut SDL_Joystick, axis: c_int) ->
              int16_t;
    pub fn SDL_JoystickGetHat(joystick: *mut SDL_Joystick, hat: c_int) ->
              int8_t;
    pub fn SDL_JoystickGetBall(joystick: *mut SDL_Joystick, ball: c_int,
                                     dx: *const c_int, dy: *const c_int) -> c_int;
    pub fn SDL_JoystickGetButton(joystick: *mut SDL_Joystick, button: c_int)
              -> uint8_t;
    pub fn SDL_JoystickClose(joystick: *mut SDL_Joystick);
}
