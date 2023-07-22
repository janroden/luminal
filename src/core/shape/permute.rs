use crate::shape::*;

pub trait PermuteShapeTo<Dst, Ax> {}

#[rustfmt::skip]
macro_rules! d { (0) => { D1 }; (1) => { D2 }; (2) => { D3 }; (3) => { D4 }; (4) => { D5 }; (5) => { D6 }; }

macro_rules! impl_permute {
    ($Ax0:tt, $Ax1:tt) => {
        impl<const D1: usize, const D2: usize>
            PermuteShapeTo<(Const<d!($Ax0)>, Const<d!($Ax1)>), Axes2<$Ax0, $Ax1>>
            for (Const<D1>, Const<D2>)
        {
        }
    };
    ($Ax0:tt, $Ax1:tt, $Ax2:tt) => {
        impl<const D1: usize, const D2: usize, const D3: usize>
            PermuteShapeTo<
                (Const<d!($Ax0)>, Const<d!($Ax1)>, Const<d!($Ax2)>),
                Axes3<$Ax0, $Ax1, $Ax2>,
            > for (Const<D1>, Const<D2>, Const<D3>)
        {
        }
    };
}

/// Expand out all the possible permutations for 2-4d
macro_rules! permutations {
    ([$Ax0:tt, $Ax1:tt]) => {
        impl_permute!($Ax1, $Ax0);
    };
    ([$Ax0:tt, $Ax1:tt, $Ax2:tt]) => {
        impl_permute!($Ax0, $Ax2, $Ax1);
        impl_permute!($Ax1, $Ax0, $Ax2);
        impl_permute!($Ax1, $Ax2, $Ax0);
        impl_permute!($Ax2, $Ax0, $Ax1);
        impl_permute!($Ax2, $Ax1, $Ax0);
    };

    ([$Ax0:tt, $Ax1:tt, $Ax2:tt, $Ax3:tt]) => {
        permutations!($Ax0, [$Ax1, $Ax2, $Ax3]);
        permutations!($Ax1, [$Ax0, $Ax2, $Ax3]);
        permutations!($Ax2, [$Ax0, $Ax1, $Ax3]);
        permutations!($Ax3, [$Ax0, $Ax1, $Ax2]);
    };
    ($Ax0:tt, [$Ax1:tt, $Ax2:tt, $Ax3:tt]) => {
        permutations!($Ax0, $Ax1, [$Ax2, $Ax3]);
        permutations!($Ax0, $Ax2, [$Ax1, $Ax3]);
        permutations!($Ax0, $Ax3, [$Ax1, $Ax2]);
    };
}

permutations!([0, 1]);
permutations!([0, 1, 2]);