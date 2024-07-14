use red4ext_rs::{
    types::{IScriptable, Native, ScriptClass, StackFrame, Type},
    NativeRepr, VoidPtr,
};

#[repr(C, align(8))]
#[derive(Default)]
pub struct GameInstance {
    pub _padding0: [u8; 0x18],
}

unsafe impl NativeRepr for GameInstance {
    const NAME: &'static str = "ScriptGameInstance";
}

#[repr(C)]
pub struct AudioSystem {
    pub base: IScriptable,
    pub _padding0: [u8; 0x3E0],
}

unsafe impl ScriptClass for AudioSystem {
    const CLASS_NAME: &'static str = "gameGameAudioSystem";
    type Kind = Native;
}

impl AsRef<IScriptable> for AudioSystem {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

/// a `C` stack frame
///
/// see [RED4ext::CStackFrame](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Stack.hpp#L111)
#[repr(C)]
#[allow(non_snake_case)]
pub struct RewindableStackFrame {
    code: *mut i8, // 0
    pad: [u8; 0x30 - 0x8],
    data: VoidPtr,       // 30
    dataType: *mut Type, // 38
    pad2: [u8; 0x62 - 0x40],
    currentParam: u8, // 62
    useDirectData: bool,
}

type StackState = (*mut i8, VoidPtr, *mut Type);

impl RewindableStackFrame {
    pub unsafe fn state(&self) -> StackState {
        (self.code, self.data, self.dataType)
    }
    pub unsafe fn rewind(&mut self, state: StackState) {
        self.code = state.0;
        self.data = state.1;
        self.dataType = state.2;
        self.currentParam = 0;
    }
}

pub unsafe fn frame_mut(src: &mut StackFrame) -> &mut RewindableStackFrame {
    unsafe { std::mem::transmute(src) }
}
