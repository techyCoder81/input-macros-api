# Input Macros Plugin and API
This plugin and related API implements a mechanism of assigning a complex series of button inputs and stick positions to a specific macro button, such as left or right dpad. When triggered, a defined macro will override all inputs for the specified series of frames. 

### Online Disclaimer
This plugin will not function in any online matches.

## Plugin Usage
Simply place a populated `macros.json` file at the root of your SD card (the specific name `macros.json` is important), along with `libinput_macros.nro` in the skyline plugins directory. If `reload` is enabled, feel free to reload the json file at runtime with the plus (+) button.

## API Usage
Currently, this crate only exposes the macro structure definitions, so that other applications can parse the recorded macros and/or write their own. Over time various exposed functions will be added to control macro playback/recording, for novel uses like TAS and recording/playback. (Training Modpack, is that you?)

## Examples
The `ewgf.json` file in the `/examples/` folder is an example of macroing EWGF on DpadLeft and DpadRight.

See [here](https://github.com/techyCoder81/input-macros/tree/master/examples) for example files.

## JSON spec
The spec for allowed json values is as follows:
### Top Level Config
`"enabled": true/false`
- this will enable or disable the macro plugin. If no file is present, macros are disabled by default.

`"reload": true/false`
- When enabled, this allows you to reload the json file on-demand when the start button is pressed, while still in a match.

`"macros": []`
- this is a json list of macro definitions


### Macro Definitions
`"name": "my macro name"`
- this field lets you name the macro, just for convenience. It is otherwise unused.

`"trigger": {}`
- this field is the trigger info for the macro.

`"steps": []`
- this is a json list of macro steps


### Trigger Definitions
`"action": "DpadRight"`
- this is the input action that will trigger the macro. Currently, only the following buttons are allowed:
    - "DpadRight"
    - "DpadLeft"
    - "DpadUp"
    - "DpadDown"
    - "LeftStick"
    - "RightStick"
    - "ButtonA"
    - "ButtonB"
    - "ButtonX"
    - "ButtonY"
    - "ButtonZL"
    - "ButtonZR"
    - "ButtonR"
    - "ButtonL"

`"kind": "Holding"`
- this optional field represents what kind of input will trigger this macro. Options are:
    - "Holding" : If the input is currently being held, the macro will trigger (this is the default)
    - "JustPressed" : If the input was just pressed this frame, the macro will trigger
    - "JustReleased" : If the input was just released this frame (negative edge), the macro will trigger


### Step Definitions

`"hold_frames": 3`
- this optional field (defaults to 1) represents the number of frames a step will be held

`"sticks: {}`
- this optional field represents the Sticks positions to override for this frame. If not present, the stick positions will not be modified, and the player's real stick positions will be used.

`"buttons": ["Attack", "Special"]`
- this is a json list of Button names to output on this frame. The allowed values are as follows
    - Attack
    - Special
    - Jump
    - Guard
    - Catch
    - Smash
    - JumpMini
    - CStickOn
    - StockShare
    - AttackRaw
    - AppealHi
    - SpecialRaw
    - AppealLw
    - AppealSL
    - AppealSR
    - FlickJump
    - GuardHold
    - SpecialRaw2
    - SpecialAll
    - AttackAll
    - AppealAll


### Sticks Definitions
`"lstick_x": 0`
- this is the left stick X position to output on this frame (-128 to 127). This value is flipped if "auto_flip_x" is true.

`"lstick_y": 0`
- this is the left stick Y position to output on this frame (-128 to 127)

`"rstick_x": 0`
- this is the right stick X position to output on this frame (-128 to 127). This value is flipped if "auto_flip_x" is true.

`"rstick_y": 0`
- this is the right stick Y position to output on this frame (-128 to 127)
