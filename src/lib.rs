pub use cxx::UniquePtr;
mod cxx_aoflagger;
pub use cxx_aoflagger::ffi::{
    cxx_aoflagger_new, CxxAOFlagger, CxxFlagMask, CxxImageSet, CxxStrategy,
};

/// Perform the binary or operation on the flag buffer of `this_flagmask` with
/// the buffer of `other_flagmask`, storing the result in `this_flagmask`
pub fn flagmask_or(
    this_flagmask: &mut UniquePtr<CxxFlagMask>,
    other_flagmask: &UniquePtr<CxxFlagMask>,
) {
    assert_eq!(
        this_flagmask.Width(),
        other_flagmask.Width(),
        "flagmasks should have the same width."
    );
    assert_eq!(
        this_flagmask.Height(),
        other_flagmask.Height(),
        "flagmasks should have the same height."
    );
    let this_buffer: &mut [bool] = this_flagmask.pin_mut().BufferMut();
    let other_buffer: &[bool] = other_flagmask.Buffer();

    this_buffer
        .iter_mut()
        .zip(other_buffer.iter())
        .for_each(|(this_flag, other_flag)| *this_flag |= other_flag);
}

/// Set the flag buffer of `this_flagmask` from the buffer of `other_flagmask`,
/// storing the result in `this_flagmask`
pub fn flagmask_set(
    this_flagmask: &mut UniquePtr<CxxFlagMask>,
    other_flagmask: &UniquePtr<CxxFlagMask>,
) {
    assert_eq!(
        this_flagmask.Width(),
        other_flagmask.Width(),
        "flagmasks should have the same width."
    );
    assert_eq!(
        this_flagmask.Height(),
        other_flagmask.Height(),
        "flagmasks should have the same height."
    );
    let this_buffer: &mut [bool] = this_flagmask.pin_mut().BufferMut();
    let other_buffer: &[bool] = other_flagmask.Buffer();

    this_buffer
        .iter_mut()
        .zip(other_buffer.iter())
        .for_each(|(this_flag, other_flag)| *this_flag = *other_flag);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _get_this_flagmask(aoflagger: &CxxAOFlagger) -> UniquePtr<CxxFlagMask> {
        let mut this_flagmask = unsafe { aoflagger.MakeFlagMask(2, 2, false) };
        let this_buffer = this_flagmask.pin_mut().BufferMut();
        this_buffer[0] = false;
        this_buffer[1] = false;
        this_buffer[2] = true;
        this_buffer[3] = true;
        this_flagmask
    }

    fn _get_other_flagmask(aoflagger: &CxxAOFlagger) -> UniquePtr<CxxFlagMask> {
        let mut other_flagmask = unsafe { aoflagger.MakeFlagMask(2, 2, false) };
        let other_buffer = other_flagmask.pin_mut().BufferMut();
        other_buffer[0] = false;
        other_buffer[1] = true;
        other_buffer[2] = false;
        other_buffer[3] = true;
        other_flagmask
    }

    #[test]
    fn test_flagmask_or() {
        let aoflagger = unsafe { cxx_aoflagger_new() };
        let mut this_flagmask = _get_this_flagmask(&aoflagger);
        let other_flagmask = _get_other_flagmask(&aoflagger);

        flagmask_or(&mut this_flagmask, &other_flagmask);

        let this_buffer = this_flagmask.Buffer();
        assert!(!this_buffer[0]);
        assert!(this_buffer[1]);
        assert!(this_buffer[2]);
        assert!(this_buffer[3]);
    }

    #[test]
    fn test_flagmask_set() {
        let aoflagger = unsafe { cxx_aoflagger_new() };
        let mut this_flagmask = _get_this_flagmask(&aoflagger);
        let other_flagmask = _get_other_flagmask(&aoflagger);

        flagmask_set(&mut this_flagmask, &other_flagmask);

        let this_buffer = this_flagmask.Buffer();
        assert!(!this_buffer[0]);
        assert!(this_buffer[1]);
        assert!(!this_buffer[2]);
        assert!(this_buffer[3]);
    }
}
