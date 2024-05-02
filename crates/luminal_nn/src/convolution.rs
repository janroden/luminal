use luminal::prelude::*;
use rand::{thread_rng, Rng};

pub struct Conv1D<
    const CH_IN: usize,
    const CH_OUT: usize,
    const KERNEL: usize,
    const STRIDE: usize = KERNEL,
    const DILATION: usize = 0,
> {
    pub weight: GraphTensor<R3<CH_OUT, CH_IN, KERNEL>>,
}

impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNEL: usize,
        const STRIDE: usize,
        const DILATION: usize,
    > InitModule for Conv1D<CH_IN, CH_OUT, KERNEL, STRIDE, DILATION>
{
    fn initialize(cx: &mut Graph) -> Self {
        // Init weight as uniform(-1, 1)
        let mut rng = thread_rng();
        Self {
            weight: cx.named_tensor("Weight").set(
                (0..(CH_IN * CH_OUT * KERNEL))
                    .map(|_| rng.gen_range(-1_f32..1_f32))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNEL: usize,
        const STRIDE: usize,
        const DILATION: usize,
    > SerializeModule for Conv1D<CH_IN, CH_OUT, KERNEL, STRIDE, DILATION>
{
    fn serialize(&self, s: &mut luminal::module::Serializer) {
        s.tensor("weight", self.weight);
    }
}

// Single
impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNEL: usize,
        const STRIDE: usize,
        const DILATION: usize,
    > Conv1D<CH_IN, CH_OUT, KERNEL, STRIDE, DILATION>
{
    pub fn forward<const DIM_IN: usize, const DIM_OUT: usize>(
        &self,
        input: GraphTensor<R2<CH_IN, DIM_IN>>,
    ) -> GraphTensor<R2<CH_OUT, DIM_OUT>> {
        self.weight
            .dyn_reshape::<(Const<CH_OUT>, Dyn<'-'>)>(vec![CH_OUT.into(), (CH_IN * KERNEL).into()])
            .matmul(
                input
                    .pool_last_dim::<R3<CH_IN, DIM_OUT, KERNEL>>(
                        KERNEL.into(),
                        STRIDE.into(),
                        DILATION,
                    )
                    .permute::<_, Axes3<0, 2, 1>>()
                    .dyn_reshape(vec![(CH_IN * KERNEL).into(), DIM_OUT.into()]),
            )
    }
}

pub struct Conv2D<
    const CH_IN: usize,
    const CH_OUT: usize,
    const KERNELX: usize,
    const KERNELY: usize,
    const STRIDEX: usize = KERNELX,
    const STRIDEY: usize = KERNELY,
    const DILATIONX: usize = 0,
    const DILATIONY: usize = 0,
> {
    pub weight: GraphTensor<R4<CH_OUT, CH_IN, KERNELX, KERNELY>>,
}

impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNELX: usize,
        const KERNELY: usize,
        const STRIDEX: usize,
        const STRIDEY: usize,
        const DILATIONX: usize,
        const DILATIONY: usize,
    > InitModule
    for Conv2D<CH_IN, CH_OUT, KERNELX, KERNELY, STRIDEX, STRIDEY, DILATIONX, DILATIONY>
{
    fn initialize(cx: &mut Graph) -> Self {
        // Init weight as uniform(-1, 1)
        let mut rng = thread_rng();
        Self {
            weight: cx.named_tensor("Weight").set(
                (0..(CH_IN * CH_OUT * KERNELX * KERNELY))
                    .map(|_| rng.gen_range(-1_f32..1_f32))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNELX: usize,
        const KERNELY: usize,
        const STRIDEX: usize,
        const STRIDEY: usize,
        const DILATIONX: usize,
        const DILATIONY: usize,
    > SerializeModule
    for Conv2D<CH_IN, CH_OUT, KERNELX, KERNELY, STRIDEX, STRIDEY, DILATIONX, DILATIONY>
{
    fn serialize(&self, s: &mut luminal::module::Serializer) {
        s.tensor("weight", self.weight);
    }
}

// Single
impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNELX: usize,
        const KERNELY: usize,
        const STRIDEX: usize,
        const STRIDEY: usize,
        const DILATIONX: usize,
        const DILATIONY: usize,
    > Conv2D<CH_IN, CH_OUT, KERNELX, KERNELY, STRIDEX, STRIDEY, DILATIONX, DILATIONY>
{
    pub fn forward<
        const DIMX_IN: usize,
        const DIMY_IN: usize,
        const DIMX_OUT: usize,
        const DIMY_OUT: usize,
    >(
        &self,
        input: GraphTensor<R3<CH_IN, DIMX_IN, DIMY_IN>>,
    ) -> GraphTensor<R3<CH_OUT, DIMX_OUT, DIMY_OUT>> {
        let input_pooled = input
            .pool_last_dim::<R4<CH_IN, DIMX_IN, DIMY_OUT, KERNELY>>(
                KERNELY.into(),
                STRIDEY.into(),
                DILATIONY,
            )
            .permute::<_, Axes4<0, 2, 3, 1>>()
            .pool_last_dim::<R5<CH_IN, DIMY_OUT, KERNELY, DIMX_OUT, KERNELX>>(
                KERNELX.into(),
                STRIDEX.into(),
                DILATIONX,
            )
            .permute::<_, Axes5<0, 4, 2, 3, 1>>()
            .dyn_reshape::<(_, Dyn<'-'>)>(vec![
                (CH_IN * KERNELX * KERNELY).into(),
                (DIMX_OUT * DIMY_OUT).into(),
            ]);

        self.weight
            .dyn_reshape::<(Const<CH_OUT>, Dyn<'-'>)>(vec![
                CH_OUT.into(),
                (CH_IN * KERNELX * KERNELY).into(),
            ])
            .matmul(input_pooled)
            .reshape::<R3<CH_OUT, DIMX_OUT, DIMY_OUT>>()
    }
}
pub struct Conv3D<
    const CH_IN: usize,
    const CH_OUT: usize,
    const KERNELX: usize,
    const KERNELY: usize,
    const KERNELZ: usize,
    const STRIDEX: usize,
    const STRIDEY: usize,
    const STRIDEZ: usize,
    const DILATIONX: usize,
    const DILATIONY: usize,
    const DILATIONZ: usize,
    const DIMX_TIMES_DIMY_DIMZ_OUT: usize
> {
    pub weight: GraphTensor<R5<CH_OUT, CH_IN, KERNELX, KERNELY, KERNELZ>>,
}

impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNELX: usize,
        const KERNELY: usize,
        const KERNELZ: usize,
        const STRIDEX: usize,
        const STRIDEY: usize,
        const STRIDEZ: usize,
        const DILATIONX: usize,
        const DILATIONY: usize,
        const DILATIONZ: usize,
        const DIMX_TIMES_DIMY_DIMZ_OUT: usize
    > InitModule
    for Conv3D<
        CH_IN,
        CH_OUT,
        KERNELX,
        KERNELY,
        KERNELZ,
        STRIDEX,
        STRIDEY,
        STRIDEZ,
        DILATIONX,
        DILATIONY,
        DILATIONZ,
        DIMX_TIMES_DIMY_DIMZ_OUT,
    >
{
    fn initialize(cx: &mut Graph) -> Self {
        // Init weight as uniform(-1, 1)
        let mut rng = thread_rng();
        Self {
            weight: cx.named_tensor("Weight").set(
                (0..(CH_IN * CH_OUT * KERNELX * KERNELY * KERNELZ))
                    .map(|_| rng.gen_range(-1_f32..1_f32))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNELX: usize,
        const KERNELY: usize,
        const KERNELZ: usize,
        const STRIDEX: usize,
        const STRIDEY: usize,
        const STRIDEZ: usize,
        const DILATIONX: usize,
        const DILATIONY: usize,
        const DILATIONZ: usize,
        const DIMX_TIMES_DIMY_DIMZ_OUT: usize,
> SerializeModule
    for Conv3D<
        CH_IN,
        CH_OUT,
        KERNELX,
        KERNELY,
        KERNELZ,
        STRIDEX,
        STRIDEY,
        STRIDEZ,
        DILATIONX,
        DILATIONY,
        DILATIONZ,
        DIMX_TIMES_DIMY_DIMZ_OUT,
    >
{
    fn serialize(&self, s: &mut luminal::module::Serializer) {
        s.tensor("weight", self.weight);
    }
}

impl<
        const CH_IN: usize,
        const CH_OUT: usize,
        const KERNELX: usize,
        const KERNELY: usize,
        const KERNELZ: usize,
        const STRIDEX: usize,
        const STRIDEY: usize,
        const STRIDEZ: usize,
        const DILATIONX: usize,
        const DILATIONY: usize,
        const DILATIONZ: usize,
        const DIMX_TIMES_DIMY_DIMZ_OUT: usize,
    >
    Conv3D<
        CH_IN,
        CH_OUT,
        KERNELX,
        KERNELY,
        KERNELZ,
        STRIDEX,
        STRIDEY,
        STRIDEZ,
        DILATIONX,
        DILATIONY,
        DILATIONZ,
        DIMX_TIMES_DIMY_DIMZ_OUT,
    >
{
    pub fn forward<
        const DIMX_IN: usize,
        const DIMY_IN: usize,
        const DIMZ_IN: usize,
        const DIMX_OUT: usize,
        const DIMY_OUT: usize,
        const DIMZ_OUT: usize,
    >(
        &self,
        input: GraphTensor<R4<CH_IN, DIMX_IN, DIMY_IN, DIMZ_IN>>,
    ) -> GraphTensor<R4<CH_OUT, DIMX_OUT, DIMY_OUT, DIMZ_OUT>> {
        // Calculate the product of DIMX_OUT, DIMY_OUT, and DIMZ_OUT

        let input_pooled = input
            .pool_last_dim::<R5<CH_IN, DIMX_IN, DIMY_OUT, DIMZ_OUT, KERNELY>>(
                KERNELY.into(),
                STRIDEY.into(),
                DILATIONY
            )
            .permute::<_, Axes5<0, 2, 3, 4, 1>>()
            .pool_last_dim::<R6<CH_IN, DIMY_OUT, DIMZ_OUT, KERNELY, DIMX_OUT, KERNELX>>(
                KERNELX.into(),
                STRIDEX.into(),
                DILATIONX
            )
            .permute::<_, Axes6<0, 5, 3, 4, 2, 1>>()
            .reshape::<R4<CH_IN, KERNELX, KERNELY, DIMX_TIMES_DIMY_DIMZ_OUT>>()
            .pool_last_dim::<R5<CH_IN, KERNELX, KERNELY, DIMX_TIMES_DIMY_DIMZ_OUT, KERNELZ>>(
                KERNELZ.into(),
                STRIDEZ.into(),
                DILATIONZ
            )
            .permute::<_, Axes5<0, 4, 2, 3, 1>>()
            .dyn_reshape::<(_, Dyn<'-'>)>(vec![
                (CH_IN * KERNELX * KERNELY * KERNELZ).into(),
                (DIMX_OUT * DIMY_OUT * DIMZ_OUT).into(),
            ]);

        self.weight
            .dyn_reshape::<(Const<CH_OUT>, Dyn<'-'>)>(vec![
                CH_OUT.into(),
                (CH_IN*KERNELX*KERNELY*KERNELZ).into(),
            ])
            .matmul(input_pooled)
            .reshape::<R4<CH_OUT, DIMX_OUT, DIMY_OUT, DIMZ_OUT>>()
    }
}




#[cfg(test)]
mod tests {
    use super::{Conv1D, Conv2D, Conv3D};
    use luminal::{prelude::*, tests::assert_close};

    #[test]
    fn test_conv1d_simple() {
        let mut cx = Graph::new();

        const CH_IN: usize = 1;
        const CH_OUT: usize = 1;
        const KERNEL: usize = 2;
        const STRIDE: usize = KERNEL;
        const DIM_IN: usize = 6;
        const DIM_OUT: usize = ((DIM_IN - (KERNEL - 1) - 1) / STRIDE) + 1;

        let model = Conv1D::<CH_IN, CH_OUT, KERNEL>::initialize(&mut cx);
        model.weight.set([[[0.0316, -0.2057]]]);

        let inp1 = cx
            .tensor::<R2<CH_IN, DIM_IN>>()
            .set([[3., 0., 9., 6., 0., 6.]]);

        let out1 = model.forward::<DIM_IN, DIM_OUT>(inp1).retrieve();
        cx.execute();

        assert_close(&out1.data(), &[0.0948, -0.9498, -1.2342]);
    }

    #[test]
    fn test_conv1d() {
        let mut cx = Graph::new();

        const CH_IN: usize = 8;
        const CH_OUT: usize = 4;
        const KERNEL: usize = 2;
        const STRIDE: usize = 2;
        const DIM_IN: usize = 12;
        const DIM_OUT: usize = ((DIM_IN - (KERNEL - 1) - 1) / STRIDE) + 1;

        let model = Conv1D::<CH_IN, CH_OUT, KERNEL>::initialize(&mut cx);
        model.weight.set(vec![
            -0.1700, -0.2000, 0.1000, -0.0200, 0.1000, 0.0200, -0.2100, -0.2300, -0.0600, 0.1500,
            0.1200, 0.1000, 0.1800, 0.0600, -0.1700, -0.0400, 0.1000, -0.0200, -0.1700, 0.1000,
            0.1100, 0.1600, 0.2000, 0.0100, -0.0500, 0.2100, -0.0200, 0.0300, -0.0900, -0.0500,
            0.1600, 0.0400, 0.0400, -0.1700, 0.1100, 0.0600, -0.1200, -0.2300, 0.2300, -0.2100,
            -0.2200, 0.1100, -0.0100, -0.1400, 0.1700, 0.0300, 0.1000, -0.1400, -0.2100, -0.1800,
            0.2000, -0.2300, -0.1600, 0.2200, 0.0900, 0.0700, -0.1000, -0.0400, -0.0500, 0.1400,
            0.0700, -0.1200, 0.1400, 0.2200,
        ]);

        let inp1 = cx.tensor::<R2<CH_IN, DIM_IN>>();
        inp1.set(vec![
            1., 2., 6., 4., 8., 1., 6., 0., 1., 0., 6., 4., 3., 4., 9., 3., 8., 8., 5., 5., 0., 4.,
            2., 7., 6., 4., 2., 2., 8., 0., 7., 3., 0., 0., 7., 2., 3., 3., 1., 9., 5., 4., 5., 5.,
            8., 0., 0., 1., 2., 1., 8., 9., 4., 7., 7., 6., 8., 5., 0., 9., 1., 6., 0., 1., 4., 3.,
            3., 5., 8., 7., 9., 5., 6., 5., 6., 9., 7., 0., 9., 5., 6., 0., 6., 1., 2., 1., 0., 1.,
            3., 6., 8., 0., 6., 6., 3., 2.,
        ]);
        inp1.retrieve();

        let out1 = model.forward::<DIM_IN, DIM_OUT>(inp1).retrieve();
        cx.execute();

        assert_close(
            &out1.data(),
            &[
                0.7600, -0.4700, 0.0100, -0.1600, -0.1800, 2.2300, 1.7200, 0.6900, 3.5100, 3.7700,
                3.4600, 3.8100, -1.2600, -1.3900, 0.9400, 0.5300, 0.6300, -0.0400, 0.3800, -1.4900,
                -0.8800, -0.3100, 1.7500, -2.7500,
            ],
        );
    }

    #[test]
    fn test_conv2d() {
        let mut cx = Graph::new();

        const CH_IN: usize = 5;
        const CH_OUT: usize = 2;
        const KERNELX: usize = 2;
        const KERNELY: usize = 2;
        const STRIDEX: usize = KERNELX;
        const STRIDEY: usize = KERNELY;
        const DILATIONX: usize = 0;
        const DILATIONY: usize = 0;
        const DIMX_IN: usize = 16;
        const DIMX_OUT: usize = ((DIMX_IN - (DILATIONX + 1) * (KERNELX - 1) - 1) / STRIDEX) + 1;
        const DIMY_IN: usize = 9;
        const DIMY_OUT: usize = ((DIMY_IN - (DILATIONY + 1) * (KERNELY - 1) - 1) / STRIDEY) + 1;

        let inp1 = cx.tensor::<R3<CH_IN, DIMX_IN, DIMY_IN>>();
        inp1.set(vec![
            8., 8., 5., 7., 0., 6., 5., 3., 0., 7., 0., 6., 6., 7., 7., 5., 0., 6., 9., 4., 0., 8.,
            8., 5., 7., 6., 2., 8., 9., 5., 0., 3., 1., 1., 8., 4., 1., 1., 5., 6., 9., 3., 2., 9.,
            4., 7., 1., 0., 7., 7., 4., 9., 5., 0., 4., 7., 4., 7., 8., 8., 4., 8., 4., 7., 9., 3.,
            7., 9., 5., 8., 5., 9., 0., 9., 5., 6., 8., 9., 5., 4., 1., 9., 7., 2., 2., 7., 9., 3.,
            1., 2., 8., 4., 0., 8., 0., 5., 6., 7., 7., 4., 3., 4., 6., 8., 3., 7., 8., 8., 7., 1.,
            5., 1., 8., 0., 1., 1., 7., 3., 2., 1., 0., 4., 5., 4., 3., 2., 5., 4., 2., 4., 1., 9.,
            4., 1., 9., 7., 7., 1., 2., 6., 3., 4., 1., 1., 6., 6., 8., 2., 7., 7., 9., 0., 9., 0.,
            1., 4., 2., 4., 9., 6., 8., 6., 1., 6., 3., 8., 3., 4., 5., 0., 2., 1., 8., 2., 2., 8.,
            7., 0., 7., 7., 3., 4., 5., 0., 7., 2., 1., 1., 4., 2., 9., 9., 6., 1., 5., 4., 6., 9.,
            5., 4., 1., 9., 1., 5., 5., 5., 8., 8., 0., 1., 3., 0., 8., 8., 5., 1., 6., 1., 5., 6.,
            4., 4., 4., 0., 1., 1., 5., 1., 7., 2., 3., 5., 5., 4., 9., 1., 3., 7., 6., 7., 1., 5.,
            3., 8., 6., 6., 6., 7., 3., 2., 2., 8., 1., 3., 0., 2., 7., 6., 5., 7., 5., 7., 8., 1.,
            2., 2., 5., 0., 2., 9., 1., 5., 3., 8., 7., 9., 7., 2., 8., 8., 8., 6., 3., 2., 7., 7.,
            0., 3., 7., 8., 3., 7., 2., 3., 2., 7., 5., 5., 6., 0., 9., 0., 9., 9., 1., 8., 7., 9.,
            6., 8., 7., 5., 4., 9., 5., 6., 3., 2., 8., 3., 0., 6., 3., 8., 3., 1., 8., 7., 2., 0.,
            7., 7., 7., 7., 8., 0., 4., 9., 8., 2., 0., 4., 4., 3., 5., 5., 3., 0., 3., 6., 3., 1.,
            2., 9., 9., 6., 8., 1., 2., 6., 8., 6., 0., 0., 2., 8., 8., 5., 0., 5., 9., 0., 8., 1.,
            1., 3., 5., 9., 3., 5., 8., 6., 3., 2., 9., 4., 8., 3., 9., 5., 2., 9., 0., 1., 6., 8.,
            0., 3., 0., 1., 2., 1., 0., 1., 4., 1., 1., 0., 6., 9., 2., 7., 2., 6., 0., 4., 8., 2.,
            6., 7., 2., 2., 7., 4., 5., 8., 1., 4., 7., 5., 9., 7., 2., 5., 9., 1., 6., 1., 7., 9.,
            5., 6., 9., 3., 5., 1., 6., 1., 3., 3., 9., 3., 9., 0., 1., 8., 1., 9., 8., 5., 3., 4.,
            4., 1., 5., 5., 4., 4., 5., 8., 7., 1., 1., 7., 3., 9., 0., 1., 3., 4., 8., 4., 0., 5.,
            6., 2., 0., 7., 8., 2., 6., 2., 9., 6., 2., 0., 3., 7., 5., 7., 1., 8., 5., 5., 9., 1.,
            0., 3., 5., 7., 5., 3., 2., 8., 6., 3., 0., 5., 8., 5., 7., 8., 8., 2., 9., 0., 1., 8.,
            6., 0., 3., 2., 5., 2., 9., 8., 9., 6., 2., 0., 3., 2., 5., 9., 1., 3., 6., 5., 2., 8.,
            2., 2., 1., 8., 6., 4., 1., 6., 0., 7., 3., 0., 9., 6., 5., 5., 5., 2., 4., 2., 8., 3.,
            0., 6., 3., 8., 8., 4., 9., 4., 7., 0., 3., 5., 1., 4., 6., 0., 0., 5., 9., 7., 8., 6.,
            7., 0., 6., 7., 0., 5., 8., 8., 6., 4., 6., 0., 2., 3., 2., 8., 7., 5., 9., 6., 6., 2.,
            0., 4., 4., 4., 4., 2., 7., 5., 3., 2., 6., 3., 7., 0., 7., 2., 5., 1., 4., 4., 5., 1.,
            6., 7., 5., 7., 0., 7., 8., 4., 7., 3., 9., 1., 7., 5., 6., 1., 0., 2., 0., 0., 5., 5.,
            8., 8., 7., 3., 7., 2., 9., 3., 8., 4., 5., 3., 8., 5., 2., 0., 2., 0., 5., 9., 0., 3.,
            8., 0., 4., 1., 8., 4., 8., 9., 1., 1., 4., 5., 0., 2., 0., 9., 4., 2., 3., 9., 0., 7.,
            3., 1., 5., 9., 1., 6., 5., 4., 2., 1., 2., 1., 1., 4., 7., 2.,
        ]);

        let exp_out1 = cx.tensor::<R3<CH_OUT, DIMX_OUT, DIMY_OUT>>();
        exp_out1.set(vec![
            3.9600, -0.3300, -1.7800, 4.0400, 1.5300, 0.2900, 2.8700, 3.0000, 0.9600, -1.8700,
            4.5900, 3.9700, 1.2800, 1.1800, 3.7800, 2.8500, 0.5500, 0.5600, 3.9800, 1.3200,
            -0.7100, -0.6500, 4.3900, 0.4000, 1.0300, 0.9800, 3.1200, 2.7400, 2.5100, 0.1200,
            1.8500, 2.0000, -0.7900, 1.0700, -0.3900, -0.8100, -2.5100, -2.9700, 0.2100, 1.8400,
            -0.7700, -0.3900, 1.2200, 0.1900, 4.1700, -4.3600, -1.8600, 0.4800, -2.4400, 2.6300,
            1.5000, -1.9700, 1.2800, -2.8200, -2.3200, 0.2200, -0.3800, 2.1800, -0.8200, -1.5700,
            1.2000, -3.4200, -1.6700, 0.9000,
        ]);

        exp_out1.retrieve();

        let model: Conv2D<CH_IN, CH_OUT, KERNELX, KERNELY> = Conv2D::initialize(&mut cx);
        model.weight.set(vec![
            0.1600, 0.2000, 0.1900, -0.1100, 0.0100, -0.0300, -0.1200, -0.0800, -0.1300, -0.0300,
            0.1600, -0.1700, -0.0000, 0.1900, 0.1300, 0.0300, -0.1500, 0.0900, 0.0100, 0.0200,
            0.1500, 0.0700, -0.0800, 0.1700, 0.1000, -0.0700, 0.1600, -0.1600, -0.1900, -0.0500,
            -0.2100, 0.0100, -0.2000, 0.2100, -0.0400, -0.1400, 0.1500, 0.0500, -0.1700, 0.1400,
        ]);

        let out1 = model
            .forward::<DIMX_IN, DIMY_IN, DIMX_OUT, DIMY_OUT>(inp1)
            .retrieve();

        cx.execute();

        assert_close(&out1.data(), &exp_out1.data())
    }

    #[test]
    fn test_conv3d() {
        let mut cx = Graph::new();

        const CH_IN: usize = 5;
        const CH_OUT: usize = 2;
        const KERNELX: usize = 2;
        const KERNELY: usize = 2;
        const KERNELZ: usize = 2;
        const STRIDEX: usize = 2;
        const STRIDEY: usize = 2;
        const STRIDEZ: usize = 2;
        const DILATIONX: usize = 0;
        const DILATIONY: usize = 0;
        const DILATIONZ: usize = 0;
        const DIMX_IN: usize = 16;
        const DIMY_IN: usize = 9;
        const DIMZ_IN: usize = 5;
        const DIMX_OUT: usize = ((DIMX_IN - (DILATIONX + 1) * (KERNELX - 1) - 1) / STRIDEX) + 1;
        const DIMY_OUT: usize = ((DIMY_IN - (DILATIONY + 1) * (KERNELY - 1) - 1) / STRIDEY) + 1;
        const DIMZ_OUT: usize = ((DIMZ_IN - (DILATIONZ + 1) * (KERNELZ - 1) - 1) / STRIDEZ) + 1;
        const DIMX_TIMES_DIMY_DIMZ_OUT:usize = DIMX_OUT * DIMY_OUT * DIMZ_OUT;
        let inp1 = cx.tensor::<R4<CH_IN, DIMX_IN, DIMY_IN, DIMZ_IN>>();
        let inp1 = cx.tensor::<R4<CH_IN, DIMX_IN, DIMY_IN, DIMZ_IN>>();
        inp1.set(vec![
            // Example input data (5 channels, 16x9x5 volume)
            8., 8., 5., 7., 0., 6., 5., 3., 0., 7., 0., 6., 6., 7., 7., 5., 0., 6., 9., 4., 0., 8.,
            8., 5., 7., 6., 2., 8., 9., 5., 0., 3., 1., 1., 8., 4., 1., 1., 5., 6., 9., 3., 2., 9.,
            4., 7., 1., 0., 7., 7., 4., 9., 5., 0., 4., 7., 4., 7., 8., 8., 4., 8., 4., 7., 9., 3.,
            7., 9., 5., 8., 5., 9., 0., 9., 5., 6., 8., 9., 5., 4., 1., 9., 7., 2., 2., 7., 9., 3.,
            1., 2., 8., 4., 0., 8., 0., 5., 6., 7., 7., 4., 3., 4., 6., 8., 3., 7., 8., 8., 7., 1.,
            5., 1., 8., 0., 1., 1., 7., 3., 2., 1., 0., 4., 5., 4., 3., 2., 5., 4., 2., 4., 1., 9.,
            4., 1., 9., 7., 7., 1., 2., 6., 3., 4., 1., 1., 6., 6., 8., 2., 7., 7., 9., 0., 9., 0.,
            1., 4., 2., 4., 9., 6., 8., 6., 1., 6., 3., 8., 3., 4., 5., 0., 2., 1., 8., 2., 2., 8.,
            7., 0., 7., 7., 3., 4., 5., 0., 7., 2., 1., 1., 4., 2., 9., 9., 6., 1., 5., 4., 6., 9.,
            5., 4., 1., 9., 1., 5., 5., 5., 8., 8., 0., 1., 3., 0., 8., 8., 5., 1., 6., 1., 5., 6.,
            4., 4., 4., 0., 1., 1., 5., 1., 7., 2., 3., 5., 5., 4., 9., 1., 3., 7., 6., 7., 1., 5.,
            3., 8., 6., 6., 6., 7., 3., 2., 2., 8., 1., 3., 0., 2., 7., 6., 5., 7., 5., 7., 8., 1.,
            2., 2., 5., 0., 2., 9., 1., 5., 3., 8., 7., 9., 7., 2., 8., 8., 8., 6., 3., 2., 7., 7.,
            0., 3., 7., 8., 3., 7., 2., 3., 2., 7., 5., 5., 6., 0., 9., 0., 9., 9., 1., 8., 7., 9.,
            6., 8., 7., 5., 4., 9., 5., 6., 3., 2., 8., 3., 0., 6., 3., 8., 3., 1., 8., 7., 2., 0.,
            7., 7., 7., 7., 8., 0., 4., 9., 8., 2., 0., 4., 4., 3., 5., 5., 3., 0., 3., 6., 3., 1.,
            2., 9., 9., 6., 8., 1., 2., 6., 8., 6., 0., 0., 2., 8., 8., 5., 0., 5., 9., 0., 8., 1.,
            1., 3., 5., 9., 3., 5., 8., 6., 3., 2., 9., 4., 8., 3., 9., 5., 2., 9., 0., 1., 6., 8.,
            0., 3., 0., 1., 2., 1., 0., 1., 4., 1., 1., 0., 6., 9., 2., 7., 2., 6., 0., 4., 8., 2.,
            6., 7., 2., 2., 7., 4., 5., 8., 1., 4., 7., 5., 9., 7., 2., 5., 9., 1., 6., 1., 7., 9.,
            5., 6., 9., 3., 5., 1., 6., 1., 3., 3., 9., 3., 9., 0., 1., 8., 1., 9., 8., 5., 3., 4.,
            4., 1., 5., 5., 4., 4., 5., 8., 7., 1., 1., 7., 3., 9., 0., 1., 3., 4., 8., 4., 0., 5.,
            6., 2., 0., 7., 8., 2., 6., 2., 9., 6., 2., 0., 3., 7., 5., 7., 1., 8., 5., 5., 9., 1.,
            0., 3., 5., 7., 5., 3., 2., 8., 6., 3., 0., 5., 8., 5., 7., 8., 8., 2., 9., 0., 1., 8.,
            6., 0., 3., 2., 5., 2., 9., 8., 9., 6., 2., 0., 3., 2., 5., 9., 1., 3., 6., 5., 2., 8.,
            2., 2., 1., 8., 6., 4., 1., 6., 0., 7., 3., 0., 9., 6., 5., 5., 5., 2., 4., 2., 8., 3.,
            0., 6., 3., 8., 8., 4., 9., 4., 7., 0., 3., 5., 1., 4., 6., 0., 0., 5., 9., 7., 8., 6.,
            7., 0., 6., 7., 0., 5., 8., 8., 6., 4., 6., 0., 2., 3., 2., 8., 7., 5., 9., 6., 6., 2.,
            0., 4., 4., 4., 4., 2., 7., 5., 3., 2., 6., 3., 7., 0., 7., 2., 5., 1., 4., 4., 5., 1.,
            6., 7., 5., 7., 0., 7., 8., 4., 7., 3., 9., 1., 7., 5., 6., 1., 0., 2., 0., 0., 5., 5.,
            8., 8., 7., 3., 7., 2., 9., 3., 8., 4., 5., 3., 8., 5., 2., 0., 2., 0., 5., 9., 0., 3.,
            8., 0., 4., 1., 8., 4., 8., 9., 1., 1., 4., 5., 0., 2., 0., 9., 4., 2., 3., 9., 0., 7.,
            3., 1., 5., 9., 1., 6., 5., 4., 2., 1., 2., 1., 1., 4., 7., 2.,
        ]);

        let exp_out1 = cx.tensor::<R4<CH_OUT, DIMX_OUT, DIMY_OUT, DIMZ_OUT>>();
        exp_out1.set(vec![
            // Example expected output data (2 channels, 8x5x2 volume)
            3.9600, -0.3300, -1.7800, 4.0400, 1.5300, 0.2900, 2.8700, 3.0000, 0.9600, -1.8700,
            4.5900, 3.9700, 1.2800, 1.1800, 3.7800, 2.8500, 0.5500, 0.5600, 3.9800, 1.3200,
            -0.7100, -0.6500, 4.3900, 0.4000, 1.0300, 0.9800, 3.1200, 2.7400, 2.5100, 0.1200,
            1.8500, 2.0000, -0.7900, 1.0700, -0.3900, -0.8100, -2.5100, -2.9700, 0.2100, 1.8400,
            -0.7700, -0.3900, 1.2200, 0.1900, 4.1700, -4.3600, -1.8600, 0.4800, -2.4400, 2.6300,
            1.5000, -1.9700, 1.2800, -2.8200, -2.3200, 0.2200, -0.3800, 2.1800, -0.8200, -1.5700,
            1.2000, -3.4200, -1.6700, 0.9000,
        ]);

        exp_out1.retrieve();

        let model: Conv3D<
            CH_IN,
            CH_OUT,
            KERNELX,
            KERNELY,
            KERNELZ,
            STRIDEX,
            STRIDEY,
            STRIDEZ,
            DILATIONX,
            DILATIONY,
            DILATIONZ,
            DIMX_TIMES_DIMY_DIMZ_OUT,

        > = Conv3D::initialize(&mut cx);
        let weights = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0,
            19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0,
            36.0,
            37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0,
            54.0,
            55.0, 56.0, 57.0, 58.0, 59.0, 60.0, 61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0, 68.0, 69.0, 70.0, 71.0,
            72.0,
        ];
        model.weight.set(weights);

        println!("=====model built");
        println!("=====model built");
        println!("=====model built");
        let out1 = model
            .forward::<DIMX_IN, DIMY_IN, DIMZ_IN, DIMX_OUT, DIMY_OUT, DIMZ_OUT>(inp1)
            .retrieve();

        println!("=========forward");
        println!("=========forward");
        println!("=========forward");

        cx.execute();

        assert_close(&out1.data(), &exp_out1.data());
    }
}
