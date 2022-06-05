use rustc_middle::mir;
use rustc_target::spec::abi::Abi;

use crate::*;
use shims::unix::dlsym as unix;
use shims::windows::dlsym as windows;

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Dlsym {
    Posix(unix::Dlsym),
    Windows(windows::Dlsym),
}

impl Dlsym {
    // Returns an error for unsupported symbols, and None if this symbol
    // should become a NULL pointer (pretend it does not exist).
    pub fn from_str<'tcx>(name: &[u8], target_os: &str) -> InterpResult<'tcx, Option<Dlsym>> {
        let name = &*String::from_utf8_lossy(name);
        Ok(match target_os {
            "linux" | "macos" => unix::Dlsym::from_str(name, target_os)?.map(Dlsym::Posix),
            "windows" => windows::Dlsym::from_str(name)?.map(Dlsym::Windows),
            os => bug!("dlsym not implemented for target_os {}", os),
        })
    }
}

impl<'mir, 'tcx: 'mir> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
pub trait EvalContextExt<'mir, 'tcx: 'mir>: crate::MiriEvalContextExt<'mir, 'tcx> {
    fn call_dlsym(
        &mut self,
        dlsym: Dlsym,
        abi: Abi,
        args: &[OpTy<'tcx, Tag>],
        dest: &PlaceTy<'tcx, Tag>,
        ret: Option<mir::BasicBlock>,
    ) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        match dlsym {
            Dlsym::Posix(dlsym) =>
                unix::EvalContextExt::call_dlsym(this, dlsym, abi, args, dest, ret),
            Dlsym::Windows(dlsym) =>
                windows::EvalContextExt::call_dlsym(this, dlsym, abi, args, dest, ret),
        }
    }
}
