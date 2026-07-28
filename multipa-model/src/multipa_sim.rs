#![allow(clippy::all)]
// Generated from ONNX "../model/multipa_sim.onnx" by burn-onnx
use burn::nn::LayerNorm;
use burn::nn::LayerNormConfig;
use burn::nn::Linear;
use burn::nn::LinearConfig;
use burn::nn::PaddingConfig1d;
use burn::nn::conv::Conv1d;
use burn::nn::conv::Conv1dConfig;
use burn::prelude::*;
use burn::tensor::Bytes;
use burn_store::BurnpackStore;
use burn_store::ModuleSnapshot;

#[derive(Module, Debug)]
pub struct Submodule1<B: Backend> {
    conv1d1: Conv1d<B>,
    layernormalization1: LayerNorm<B>,
    conv1d2: Conv1d<B>,
    layernormalization2: LayerNorm<B>,
    conv1d3: Conv1d<B>,
    layernormalization3: LayerNorm<B>,
    conv1d4: Conv1d<B>,
    layernormalization4: LayerNorm<B>,
    conv1d5: Conv1d<B>,
    layernormalization5: LayerNorm<B>,
    conv1d6: Conv1d<B>,
    layernormalization6: LayerNorm<B>,
    conv1d7: Conv1d<B>,
    layernormalization7: LayerNorm<B>,
    layernormalization8: LayerNorm<B>,
    linear1: Linear<B>,
    conv1d8: Conv1d<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule1<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let conv1d1 = Conv1dConfig::new(1, 512, 10)
            .with_stride(5)
            .with_padding(PaddingConfig1d::Valid)
            .with_dilation(1)
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let layernormalization1 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let conv1d2 = Conv1dConfig::new(512, 512, 3)
            .with_stride(2)
            .with_padding(PaddingConfig1d::Valid)
            .with_dilation(1)
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let layernormalization2 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let conv1d3 = Conv1dConfig::new(512, 512, 3)
            .with_stride(2)
            .with_padding(PaddingConfig1d::Valid)
            .with_dilation(1)
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let layernormalization3 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let conv1d4 = Conv1dConfig::new(512, 512, 3)
            .with_stride(2)
            .with_padding(PaddingConfig1d::Valid)
            .with_dilation(1)
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let layernormalization4 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let conv1d5 = Conv1dConfig::new(512, 512, 3)
            .with_stride(2)
            .with_padding(PaddingConfig1d::Valid)
            .with_dilation(1)
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let layernormalization5 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let conv1d6 = Conv1dConfig::new(512, 512, 2)
            .with_stride(2)
            .with_padding(PaddingConfig1d::Valid)
            .with_dilation(1)
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let layernormalization6 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let conv1d7 = Conv1dConfig::new(512, 512, 2)
            .with_stride(2)
            .with_padding(PaddingConfig1d::Valid)
            .with_dilation(1)
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let layernormalization7 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let layernormalization8 = LayerNormConfig::new(512)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear1 = LinearConfig::new(512, 1024).with_bias(true).init(device);
        let conv1d8 = Conv1dConfig::new(1024, 1024, 128)
            .with_stride(1)
            .with_padding(PaddingConfig1d::Explicit(64, 64))
            .with_dilation(1)
            .with_groups(16)
            .with_bias(true)
            .init(device);
        Self {
            conv1d1,
            layernormalization1,
            conv1d2,
            layernormalization2,
            conv1d3,
            layernormalization3,
            conv1d4,
            layernormalization4,
            conv1d5,
            layernormalization5,
            conv1d6,
            layernormalization6,
            conv1d7,
            layernormalization7,
            layernormalization8,
            linear1,
            conv1d8,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        input_values: Tensor<B, 2>,
    ) -> (Tensor<B, 3>, [i64; 1], i64, Tensor<B, 4, Bool>, [i64; 1]) {
        let shape1_out1: [i64; 1] = {
            let axes = &input_values.clone().dims()[0..1];
            let mut output = [0i64; 1];
            for i in 0..1 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let shape2_out1: [i64; 1] = {
            let axes = &input_values.clone().dims()[1..2];
            let mut output = [0i64; 1];
            for i in 0..1 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let squeeze1_out1 = shape2_out1[0] as i64;
        let constant357_out1 = 80i64;
        let div1_out1 = squeeze1_out1 / constant357_out1;
        let constant358_out1 = -3i64;
        let add1_out1 = constant358_out1 + div1_out1;
        let constant359_out1 = 2i64;
        let div2_out1 = add1_out1 / constant359_out1;
        let constant356_out1 = -1i64;
        let add2_out1 = constant356_out1 + div2_out1;
        let div3_out1 = add2_out1 / constant359_out1;
        let constant278_out1 = 1i64;
        let add3_out1 = constant278_out1 + div3_out1;
        let unsqueeze1_out1: Tensor<B, 3> = input_values.unsqueeze_dims::<3>(&[1]);
        let conv1d1_out1 = self.conv1d1.forward(unsqueeze1_out1);
        let transpose1_out1 = conv1d1_out1.permute([0, 2, 1]);
        let layernormalization1_out1 = {
            let dtype = transpose1_out1.dtype();
            self.layernormalization1
                .forward(transpose1_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let transpose2_out1 = layernormalization1_out1.permute([0, 2, 1]);
        let gelu1_out1 = burn::tensor::activation::gelu(transpose2_out1);
        let conv1d2_out1 = self.conv1d2.forward(gelu1_out1);
        let transpose3_out1 = conv1d2_out1.permute([0, 2, 1]);
        let layernormalization2_out1 = {
            let dtype = transpose3_out1.dtype();
            self.layernormalization2
                .forward(transpose3_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let transpose4_out1 = layernormalization2_out1.permute([0, 2, 1]);
        let gelu2_out1 = burn::tensor::activation::gelu(transpose4_out1);
        let conv1d3_out1 = self.conv1d3.forward(gelu2_out1);
        let transpose5_out1 = conv1d3_out1.permute([0, 2, 1]);
        let layernormalization3_out1 = {
            let dtype = transpose5_out1.dtype();
            self.layernormalization3
                .forward(transpose5_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let transpose6_out1 = layernormalization3_out1.permute([0, 2, 1]);
        let gelu3_out1 = burn::tensor::activation::gelu(transpose6_out1);
        let conv1d4_out1 = self.conv1d4.forward(gelu3_out1);
        let transpose7_out1 = conv1d4_out1.permute([0, 2, 1]);
        let layernormalization4_out1 = {
            let dtype = transpose7_out1.dtype();
            self.layernormalization4
                .forward(transpose7_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let transpose8_out1 = layernormalization4_out1.permute([0, 2, 1]);
        let gelu4_out1 = burn::tensor::activation::gelu(transpose8_out1);
        let conv1d5_out1 = self.conv1d5.forward(gelu4_out1);
        let transpose9_out1 = conv1d5_out1.permute([0, 2, 1]);
        let layernormalization5_out1 = {
            let dtype = transpose9_out1.dtype();
            self.layernormalization5
                .forward(transpose9_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let transpose10_out1 = layernormalization5_out1.permute([0, 2, 1]);
        let gelu5_out1 = burn::tensor::activation::gelu(transpose10_out1);
        let conv1d6_out1 = self.conv1d6.forward(gelu5_out1);
        let transpose11_out1 = conv1d6_out1.permute([0, 2, 1]);
        let layernormalization6_out1 = {
            let dtype = transpose11_out1.dtype();
            self.layernormalization6
                .forward(transpose11_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let transpose12_out1 = layernormalization6_out1.permute([0, 2, 1]);
        let gelu6_out1 = burn::tensor::activation::gelu(transpose12_out1);
        let conv1d7_out1 = self.conv1d7.forward(gelu6_out1);
        let transpose13_out1 = conv1d7_out1.permute([0, 2, 1]);
        let layernormalization7_out1 = {
            let dtype = transpose13_out1.dtype();
            self.layernormalization7
                .forward(transpose13_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let transpose14_out1 = layernormalization7_out1.permute([0, 2, 1]);
        let gelu7_out1 = burn::tensor::activation::gelu(transpose14_out1);
        let transpose15_out1 = gelu7_out1.permute([0, 2, 1]);
        let layernormalization8_out1 = {
            let dtype = transpose15_out1.dtype();
            self.layernormalization8
                .forward(transpose15_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear1_out1 = self.linear1.forward(layernormalization8_out1);
        let range1_out1 = {
            let __start = 0i64;
            let __limit = add3_out1;
            let __delta = 1i64;
            assert!(__delta != 0);
            let __n = ((__limit - __start) as f64 / __delta as f64)
                .ceil()
                .max(0.0) as i64;
            Tensor::arange(0..__n, &self.device)
                .cast(burn::tensor::DType::I64)
                .mul_scalar(__delta)
                .add_scalar(__start)
        };
        let unsqueeze2_out1: Tensor<B, 4, Int> = range1_out1.unsqueeze_dims::<4>(&[0, 1, 3]);
        let constant277_out1 = 0i64;
        let greaterorequal1_out1 = unsqueeze2_out1.greater_equal_elem(constant277_out1);
        let constant360_out1: [i64; 1] = [1i64];
        let concat1_out1: [i64; 4usize] = [
            &shape1_out1[..],
            &constant360_out1[..],
            &[add3_out1][..],
            &[add3_out1][..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let expand1_out1 = {
            let onnx_shape: [i64; 4usize] = concat1_out1;
            let input_dims = greaterorequal1_out1.dims();
            let mut shape = onnx_shape;
            #[allow(clippy::needless_range_loop)]
            for i in 0..4usize {
                let dim_offset = 4usize - 4usize + i;
                if shape[dim_offset] == 1 && input_dims[i] > 1 {
                    shape[dim_offset] = input_dims[i] as i64;
                }
            }
            greaterorequal1_out1.expand(shape)
        };
        let transpose16_out1 = linear1_out1.clone().permute([0, 2, 1]);
        let conv1d8_out1 = self.conv1d8.forward(transpose16_out1);
        let slice1_out1 = conv1d8_out1.slice(s![.., .., 0..-1]);
        let gelu8_out1 = burn::tensor::activation::gelu(slice1_out1);
        let transpose17_out1 = gelu8_out1.permute([0, 2, 1]);
        let add4_out1 = linear1_out1.add(transpose17_out1);
        (
            add4_out1,
            shape1_out1,
            add3_out1,
            expand1_out1,
            constant360_out1,
        )
    }
}
#[derive(Module, Debug)]
pub struct Submodule2<B: Backend> {
    layernormalization9: LayerNorm<B>,
    linear2: Linear<B>,
    constant37: burn::module::Param<Tensor<B, 1>>,
    constant35: burn::module::Param<Tensor<B, 1>>,
    constant36: burn::module::Param<Tensor<B, 1>>,
    constant282: burn::module::Param<Tensor<B, 1>>,
    linear3: Linear<B>,
    layernormalization10: LayerNorm<B>,
    linear4: Linear<B>,
    linear5: Linear<B>,
    layernormalization11: LayerNorm<B>,
    linear6: Linear<B>,
    constant47: burn::module::Param<Tensor<B, 1>>,
    constant45: burn::module::Param<Tensor<B, 1>>,
    constant46: burn::module::Param<Tensor<B, 1>>,
    linear7: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule2<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization9 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear2 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant37: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant35: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant36: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant282: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::from_data(
                    burn::tensor::TensorData::from([0.3535533845424652f64]),
                    (device, burn::tensor::DType::F32),
                )
            },
            device.clone(),
            false,
            [1].into(),
        );
        let linear3 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization10 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear4 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear5 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization11 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear6 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant47: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant45: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant46: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear7 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization9,
            linear2,
            constant37,
            constant35,
            constant36,
            constant282,
            linear3,
            layernormalization10,
            linear4,
            linear5,
            layernormalization11,
            linear6,
            constant47,
            constant45,
            constant46,
            linear7,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add4_out1: Tensor<B, 3>,
        shape1_out1: [i64; 1],
        add3_out1: i64,
        expand1_out1: Tensor<B, 4, Bool>,
        constant360_out1: [i64; 1],
    ) -> (
        Tensor<B, 3>,
        [i64; 4],
        [i64; 1],
        [i64; 1],
        Tensor<B, 1>,
        Tensor<B, 4>,
        f32,
        [i64; 3],
    ) {
        let layernormalization9_out1 = {
            let dtype = add4_out1.clone().dtype();
            self.layernormalization9
                .forward(add4_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear2_out1 = self.linear2.forward(layernormalization9_out1);
        let split_tensors = linear2_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split1_out1, split1_out2, split1_out3] = split_tensors.try_into().unwrap();
        let constant37_out1 = self.constant37.val();
        let add5_out1 = split1_out1.add((constant37_out1).unsqueeze_dims(&[0isize, 1isize]));
        let constant280_out1: [i64; 1] = [-1i64];
        let constant361_out1: [i64; 1] = [64i64];
        let concat2_out1: [i64; 4usize] = [
            &shape1_out1[..],
            &[add3_out1][..],
            &constant280_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape2_out1 = add5_out1.reshape(concat2_out1);
        let transpose18_out1 = reshape2_out1.permute([0, 2, 1, 3]);
        let constant35_out1 = self.constant35.val();
        let add6_out1 = split1_out2.add((constant35_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape3_out1 = add6_out1.reshape(concat2_out1);
        let transpose19_out1 = reshape3_out1.permute([0, 2, 1, 3]);
        let constant36_out1 = self.constant36.val();
        let add7_out1 = split1_out3.add((constant36_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape4_out1 = add7_out1.reshape(concat2_out1);
        let transpose20_out1 = reshape4_out1.permute([0, 2, 1, 3]);
        let shape3_out1: [i64; 4] = {
            let axes = &transpose19_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice2_out1: [i64; 1] = shape3_out1[2..3].try_into().unwrap();
        let slice3_out1: [i64; 2] = shape3_out1[0..2].try_into().unwrap();
        let concat3_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice2_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape5_out1 = transpose19_out1.reshape(concat3_out1);
        let transpose21_out1 = reshape5_out1.permute([0, 2, 1]);
        let concat4_out1: [i64; 4usize] =
            [&slice3_out1[..], &constant361_out1[..], &slice2_out1[..]]
                .concat()
                .try_into()
                .unwrap();
        let reshape6_out1 = transpose21_out1.reshape(concat4_out1);
        let constant282_out1 = self.constant282.val();
        let mul1_out1 = transpose18_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul2_out1 =
            reshape6_out1.mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let constant364_out1 = 0f32;
        let constant365_out1 = -340282350000000000000000000000000000000f32;
        let where1_out1 = {
            let cond = expand1_out1;
            Tensor::<B, 1>::from_data(
                burn::tensor::TensorData::from([constant365_out1 as f64]),
                (&self.device, burn::tensor::DType::F32),
            )
            .reshape([1, 1, 1, 1])
            .expand(cond.dims())
            .mask_fill(cond, constant364_out1)
        };
        let matmul3_out1 = mul1_out1.matmul(mul2_out1);
        let add8_out1 = matmul3_out1.add(where1_out1.clone());
        let softmax1_out1 = burn::tensor::activation::softmax(add8_out1, 3);
        let isnan1_out1 = softmax1_out1.clone().is_nan();
        let where2_out1 = softmax1_out1.mask_fill(isnan1_out1, constant364_out1);
        let matmul4_out1 = where2_out1.matmul(transpose20_out1);
        let transpose22_out1 = matmul4_out1.permute([0, 2, 1, 3]);
        let concat5_out1: [i64; 3usize] =
            [&shape1_out1[..], &[add3_out1][..], &constant280_out1[..]]
                .concat()
                .try_into()
                .unwrap();
        let reshape7_out1 = transpose22_out1.reshape(concat5_out1);
        let linear3_out1 = self.linear3.forward(reshape7_out1);
        let add9_out1 = add4_out1.add(linear3_out1);
        let layernormalization10_out1 = {
            let dtype = add9_out1.clone().dtype();
            self.layernormalization10
                .forward(add9_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear4_out1 = self.linear4.forward(layernormalization10_out1);
        let gelu9_out1 = burn::tensor::activation::gelu(linear4_out1);
        let linear5_out1 = self.linear5.forward(gelu9_out1);
        let add10_out1 = add9_out1.add(linear5_out1);
        let layernormalization11_out1 = {
            let dtype = add10_out1.clone().dtype();
            self.layernormalization11
                .forward(add10_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear6_out1 = self.linear6.forward(layernormalization11_out1);
        let split_tensors = linear6_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split2_out1, split2_out2, split2_out3] = split_tensors.try_into().unwrap();
        let constant47_out1 = self.constant47.val();
        let add11_out1 = split2_out1.add((constant47_out1).unsqueeze_dims(&[0isize, 1isize]));
        let concat6_out1: [i64; 4usize] = [
            &constant360_out1[..],
            &[add3_out1][..],
            &constant280_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape8_out1 = add11_out1.reshape(concat6_out1);
        let transpose23_out1 = reshape8_out1.permute([0, 2, 1, 3]);
        let constant45_out1 = self.constant45.val();
        let add12_out1 = split2_out2.add((constant45_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape9_out1 = add12_out1.reshape(concat6_out1);
        let transpose24_out1 = reshape9_out1.permute([0, 2, 1, 3]);
        let constant46_out1 = self.constant46.val();
        let add13_out1 = split2_out3.add((constant46_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape10_out1 = add13_out1.reshape(concat6_out1);
        let transpose25_out1 = reshape10_out1.permute([0, 2, 1, 3]);
        let shape4_out1: [i64; 4] = {
            let axes = &transpose24_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice4_out1: [i64; 1] = shape4_out1[2..3].try_into().unwrap();
        let concat7_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice4_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape11_out1 = transpose24_out1.reshape(concat7_out1);
        let transpose26_out1 = reshape11_out1.permute([0, 2, 1]);
        let reshape12_out1 = transpose26_out1.reshape([1, 16, 64, -1]);
        let mul3_out1 = transpose23_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul4_out1 = reshape12_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul9_out1 = mul3_out1.matmul(mul4_out1);
        let add14_out1 = matmul9_out1.add(where1_out1.clone());
        let softmax2_out1 = burn::tensor::activation::softmax(add14_out1, 3);
        let isnan2_out1 = softmax2_out1.clone().is_nan();
        let where3_out1 = softmax2_out1.mask_fill(isnan2_out1, constant364_out1);
        let matmul10_out1 = where3_out1.matmul(transpose25_out1);
        let transpose27_out1 = matmul10_out1.permute([0, 2, 1, 3]);
        let concat8_out1: [i64; 3usize] = [
            &constant360_out1[..],
            &[add3_out1][..],
            &constant280_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape13_out1 = transpose27_out1.reshape(concat8_out1);
        let linear7_out1 = self.linear7.forward(reshape13_out1);
        let add15_out1 = add10_out1.add(linear7_out1);
        (
            add15_out1,
            concat6_out1,
            constant280_out1,
            constant361_out1,
            constant282_out1,
            where1_out1,
            constant364_out1,
            concat8_out1,
        )
    }
}
#[derive(Module, Debug)]
pub struct Submodule3<B: Backend> {
    layernormalization12: LayerNorm<B>,
    linear8: Linear<B>,
    linear9: Linear<B>,
    layernormalization13: LayerNorm<B>,
    linear10: Linear<B>,
    constant57: burn::module::Param<Tensor<B, 1>>,
    constant55: burn::module::Param<Tensor<B, 1>>,
    constant56: burn::module::Param<Tensor<B, 1>>,
    linear11: Linear<B>,
    layernormalization14: LayerNorm<B>,
    linear12: Linear<B>,
    linear13: Linear<B>,
    layernormalization15: LayerNorm<B>,
    linear14: Linear<B>,
    constant67: burn::module::Param<Tensor<B, 1>>,
    constant65: burn::module::Param<Tensor<B, 1>>,
    constant66: burn::module::Param<Tensor<B, 1>>,
    linear15: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule3<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization12 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear8 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear9 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization13 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear10 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant57: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant55: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant56: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear11 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization14 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear12 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear13 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization15 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear14 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant67: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant65: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant66: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear15 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization12,
            linear8,
            linear9,
            layernormalization13,
            linear10,
            constant57,
            constant55,
            constant56,
            linear11,
            layernormalization14,
            linear12,
            linear13,
            layernormalization15,
            linear14,
            constant67,
            constant65,
            constant66,
            linear15,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add15_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization12_out1 = {
            let dtype = add15_out1.clone().dtype();
            self.layernormalization12
                .forward(add15_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear8_out1 = self.linear8.forward(layernormalization12_out1);
        let gelu10_out1 = burn::tensor::activation::gelu(linear8_out1);
        let linear9_out1 = self.linear9.forward(gelu10_out1);
        let add16_out1 = add15_out1.add(linear9_out1);
        let layernormalization13_out1 = {
            let dtype = add16_out1.clone().dtype();
            self.layernormalization13
                .forward(add16_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear10_out1 = self.linear10.forward(layernormalization13_out1);
        let split_tensors = linear10_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split3_out1, split3_out2, split3_out3] = split_tensors.try_into().unwrap();
        let constant57_out1 = self.constant57.val();
        let add17_out1 = split3_out1.add((constant57_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape14_out1 = add17_out1.reshape(concat6_out1);
        let transpose28_out1 = reshape14_out1.permute([0, 2, 1, 3]);
        let constant55_out1 = self.constant55.val();
        let add18_out1 = split3_out2.add((constant55_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape15_out1 = add18_out1.reshape(concat6_out1);
        let transpose29_out1 = reshape15_out1.permute([0, 2, 1, 3]);
        let constant56_out1 = self.constant56.val();
        let add19_out1 = split3_out3.add((constant56_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape16_out1 = add19_out1.reshape(concat6_out1);
        let transpose30_out1 = reshape16_out1.permute([0, 2, 1, 3]);
        let shape5_out1: [i64; 4] = {
            let axes = &transpose29_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice5_out1: [i64; 1] = shape5_out1[2..3].try_into().unwrap();
        let concat9_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice5_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape17_out1 = transpose29_out1.reshape(concat9_out1);
        let transpose31_out1 = reshape17_out1.permute([0, 2, 1]);
        let reshape18_out1 = transpose31_out1.reshape([1, 16, 64, -1]);
        let mul5_out1 = transpose28_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul6_out1 = reshape18_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul15_out1 = mul5_out1.matmul(mul6_out1);
        let add20_out1 = matmul15_out1.add(where1_out1.clone());
        let softmax3_out1 = burn::tensor::activation::softmax(add20_out1, 3);
        let isnan3_out1 = softmax3_out1.clone().is_nan();
        let where4_out1 = softmax3_out1.mask_fill(isnan3_out1, constant364_out1);
        let matmul16_out1 = where4_out1.matmul(transpose30_out1);
        let transpose32_out1 = matmul16_out1.permute([0, 2, 1, 3]);
        let reshape19_out1 = transpose32_out1.reshape(concat8_out1);
        let linear11_out1 = self.linear11.forward(reshape19_out1);
        let add21_out1 = add16_out1.add(linear11_out1);
        let layernormalization14_out1 = {
            let dtype = add21_out1.clone().dtype();
            self.layernormalization14
                .forward(add21_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear12_out1 = self.linear12.forward(layernormalization14_out1);
        let gelu11_out1 = burn::tensor::activation::gelu(linear12_out1);
        let linear13_out1 = self.linear13.forward(gelu11_out1);
        let add22_out1 = add21_out1.add(linear13_out1);
        let layernormalization15_out1 = {
            let dtype = add22_out1.clone().dtype();
            self.layernormalization15
                .forward(add22_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear14_out1 = self.linear14.forward(layernormalization15_out1);
        let split_tensors = linear14_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split4_out1, split4_out2, split4_out3] = split_tensors.try_into().unwrap();
        let constant67_out1 = self.constant67.val();
        let add23_out1 = split4_out1.add((constant67_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape20_out1 = add23_out1.reshape(concat6_out1);
        let transpose33_out1 = reshape20_out1.permute([0, 2, 1, 3]);
        let constant65_out1 = self.constant65.val();
        let add24_out1 = split4_out2.add((constant65_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape21_out1 = add24_out1.reshape(concat6_out1);
        let transpose34_out1 = reshape21_out1.permute([0, 2, 1, 3]);
        let constant66_out1 = self.constant66.val();
        let add25_out1 = split4_out3.add((constant66_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape22_out1 = add25_out1.reshape(concat6_out1);
        let transpose35_out1 = reshape22_out1.permute([0, 2, 1, 3]);
        let shape6_out1: [i64; 4] = {
            let axes = &transpose34_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice6_out1: [i64; 1] = shape6_out1[2..3].try_into().unwrap();
        let concat10_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice6_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape23_out1 = transpose34_out1.reshape(concat10_out1);
        let transpose36_out1 = reshape23_out1.permute([0, 2, 1]);
        let reshape24_out1 = transpose36_out1.reshape([1, 16, 64, -1]);
        let mul7_out1 = transpose33_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul8_out1 =
            reshape24_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul21_out1 = mul7_out1.matmul(mul8_out1);
        let add26_out1 = matmul21_out1.add(where1_out1);
        let softmax4_out1 = burn::tensor::activation::softmax(add26_out1, 3);
        let isnan4_out1 = softmax4_out1.clone().is_nan();
        let where5_out1 = softmax4_out1.mask_fill(isnan4_out1, constant364_out1);
        let matmul22_out1 = where5_out1.matmul(transpose35_out1);
        let transpose37_out1 = matmul22_out1.permute([0, 2, 1, 3]);
        let reshape25_out1 = transpose37_out1.reshape(concat8_out1);
        let linear15_out1 = self.linear15.forward(reshape25_out1);
        let add27_out1 = add22_out1.add(linear15_out1);
        add27_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule4<B: Backend> {
    layernormalization16: LayerNorm<B>,
    linear16: Linear<B>,
    linear17: Linear<B>,
    layernormalization17: LayerNorm<B>,
    linear18: Linear<B>,
    constant77: burn::module::Param<Tensor<B, 1>>,
    constant75: burn::module::Param<Tensor<B, 1>>,
    constant76: burn::module::Param<Tensor<B, 1>>,
    linear19: Linear<B>,
    layernormalization18: LayerNorm<B>,
    linear20: Linear<B>,
    linear21: Linear<B>,
    layernormalization19: LayerNorm<B>,
    linear22: Linear<B>,
    constant87: burn::module::Param<Tensor<B, 1>>,
    constant85: burn::module::Param<Tensor<B, 1>>,
    constant86: burn::module::Param<Tensor<B, 1>>,
    linear23: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule4<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization16 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear16 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear17 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization17 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear18 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant77: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant75: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant76: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear19 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization18 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear20 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear21 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization19 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear22 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant87: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant85: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant86: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear23 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization16,
            linear16,
            linear17,
            layernormalization17,
            linear18,
            constant77,
            constant75,
            constant76,
            linear19,
            layernormalization18,
            linear20,
            linear21,
            layernormalization19,
            linear22,
            constant87,
            constant85,
            constant86,
            linear23,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add27_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization16_out1 = {
            let dtype = add27_out1.clone().dtype();
            self.layernormalization16
                .forward(add27_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear16_out1 = self.linear16.forward(layernormalization16_out1);
        let gelu12_out1 = burn::tensor::activation::gelu(linear16_out1);
        let linear17_out1 = self.linear17.forward(gelu12_out1);
        let add28_out1 = add27_out1.add(linear17_out1);
        let layernormalization17_out1 = {
            let dtype = add28_out1.clone().dtype();
            self.layernormalization17
                .forward(add28_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear18_out1 = self.linear18.forward(layernormalization17_out1);
        let split_tensors = linear18_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split5_out1, split5_out2, split5_out3] = split_tensors.try_into().unwrap();
        let constant77_out1 = self.constant77.val();
        let add29_out1 = split5_out1.add((constant77_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape26_out1 = add29_out1.reshape(concat6_out1);
        let transpose38_out1 = reshape26_out1.permute([0, 2, 1, 3]);
        let constant75_out1 = self.constant75.val();
        let add30_out1 = split5_out2.add((constant75_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape27_out1 = add30_out1.reshape(concat6_out1);
        let transpose39_out1 = reshape27_out1.permute([0, 2, 1, 3]);
        let constant76_out1 = self.constant76.val();
        let add31_out1 = split5_out3.add((constant76_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape28_out1 = add31_out1.reshape(concat6_out1);
        let transpose40_out1 = reshape28_out1.permute([0, 2, 1, 3]);
        let shape7_out1: [i64; 4] = {
            let axes = &transpose39_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice7_out1: [i64; 1] = shape7_out1[2..3].try_into().unwrap();
        let concat11_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice7_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape29_out1 = transpose39_out1.reshape(concat11_out1);
        let transpose41_out1 = reshape29_out1.permute([0, 2, 1]);
        let reshape30_out1 = transpose41_out1.reshape([1, 16, 64, -1]);
        let mul9_out1 = transpose38_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul10_out1 = reshape30_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul27_out1 = mul9_out1.matmul(mul10_out1);
        let add32_out1 = matmul27_out1.add(where1_out1.clone());
        let softmax5_out1 = burn::tensor::activation::softmax(add32_out1, 3);
        let isnan5_out1 = softmax5_out1.clone().is_nan();
        let where6_out1 = softmax5_out1.mask_fill(isnan5_out1, constant364_out1);
        let matmul28_out1 = where6_out1.matmul(transpose40_out1);
        let transpose42_out1 = matmul28_out1.permute([0, 2, 1, 3]);
        let reshape31_out1 = transpose42_out1.reshape(concat8_out1);
        let linear19_out1 = self.linear19.forward(reshape31_out1);
        let add33_out1 = add28_out1.add(linear19_out1);
        let layernormalization18_out1 = {
            let dtype = add33_out1.clone().dtype();
            self.layernormalization18
                .forward(add33_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear20_out1 = self.linear20.forward(layernormalization18_out1);
        let gelu13_out1 = burn::tensor::activation::gelu(linear20_out1);
        let linear21_out1 = self.linear21.forward(gelu13_out1);
        let add34_out1 = add33_out1.add(linear21_out1);
        let layernormalization19_out1 = {
            let dtype = add34_out1.clone().dtype();
            self.layernormalization19
                .forward(add34_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear22_out1 = self.linear22.forward(layernormalization19_out1);
        let split_tensors = linear22_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split6_out1, split6_out2, split6_out3] = split_tensors.try_into().unwrap();
        let constant87_out1 = self.constant87.val();
        let add35_out1 = split6_out1.add((constant87_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape32_out1 = add35_out1.reshape(concat6_out1);
        let transpose43_out1 = reshape32_out1.permute([0, 2, 1, 3]);
        let constant85_out1 = self.constant85.val();
        let add36_out1 = split6_out2.add((constant85_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape33_out1 = add36_out1.reshape(concat6_out1);
        let transpose44_out1 = reshape33_out1.permute([0, 2, 1, 3]);
        let constant86_out1 = self.constant86.val();
        let add37_out1 = split6_out3.add((constant86_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape34_out1 = add37_out1.reshape(concat6_out1);
        let transpose45_out1 = reshape34_out1.permute([0, 2, 1, 3]);
        let shape8_out1: [i64; 4] = {
            let axes = &transpose44_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice8_out1: [i64; 1] = shape8_out1[2..3].try_into().unwrap();
        let concat12_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice8_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape35_out1 = transpose44_out1.reshape(concat12_out1);
        let transpose46_out1 = reshape35_out1.permute([0, 2, 1]);
        let reshape36_out1 = transpose46_out1.reshape([1, 16, 64, -1]);
        let mul11_out1 = transpose43_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul12_out1 =
            reshape36_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul33_out1 = mul11_out1.matmul(mul12_out1);
        let add38_out1 = matmul33_out1.add(where1_out1);
        let softmax6_out1 = burn::tensor::activation::softmax(add38_out1, 3);
        let isnan6_out1 = softmax6_out1.clone().is_nan();
        let where7_out1 = softmax6_out1.mask_fill(isnan6_out1, constant364_out1);
        let matmul34_out1 = where7_out1.matmul(transpose45_out1);
        let transpose47_out1 = matmul34_out1.permute([0, 2, 1, 3]);
        let reshape37_out1 = transpose47_out1.reshape(concat8_out1);
        let linear23_out1 = self.linear23.forward(reshape37_out1);
        let add39_out1 = add34_out1.add(linear23_out1);
        add39_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule5<B: Backend> {
    layernormalization20: LayerNorm<B>,
    linear24: Linear<B>,
    linear25: Linear<B>,
    layernormalization21: LayerNorm<B>,
    linear26: Linear<B>,
    constant97: burn::module::Param<Tensor<B, 1>>,
    constant95: burn::module::Param<Tensor<B, 1>>,
    constant96: burn::module::Param<Tensor<B, 1>>,
    linear27: Linear<B>,
    layernormalization22: LayerNorm<B>,
    linear28: Linear<B>,
    linear29: Linear<B>,
    layernormalization23: LayerNorm<B>,
    linear30: Linear<B>,
    constant107: burn::module::Param<Tensor<B, 1>>,
    constant105: burn::module::Param<Tensor<B, 1>>,
    constant106: burn::module::Param<Tensor<B, 1>>,
    linear31: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule5<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization20 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear24 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear25 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization21 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear26 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant97: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant95: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant96: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear27 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization22 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear28 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear29 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization23 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear30 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant107: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant105: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant106: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear31 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization20,
            linear24,
            linear25,
            layernormalization21,
            linear26,
            constant97,
            constant95,
            constant96,
            linear27,
            layernormalization22,
            linear28,
            linear29,
            layernormalization23,
            linear30,
            constant107,
            constant105,
            constant106,
            linear31,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add39_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization20_out1 = {
            let dtype = add39_out1.clone().dtype();
            self.layernormalization20
                .forward(add39_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear24_out1 = self.linear24.forward(layernormalization20_out1);
        let gelu14_out1 = burn::tensor::activation::gelu(linear24_out1);
        let linear25_out1 = self.linear25.forward(gelu14_out1);
        let add40_out1 = add39_out1.add(linear25_out1);
        let layernormalization21_out1 = {
            let dtype = add40_out1.clone().dtype();
            self.layernormalization21
                .forward(add40_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear26_out1 = self.linear26.forward(layernormalization21_out1);
        let split_tensors = linear26_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split7_out1, split7_out2, split7_out3] = split_tensors.try_into().unwrap();
        let constant97_out1 = self.constant97.val();
        let add41_out1 = split7_out1.add((constant97_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape38_out1 = add41_out1.reshape(concat6_out1);
        let transpose48_out1 = reshape38_out1.permute([0, 2, 1, 3]);
        let constant95_out1 = self.constant95.val();
        let add42_out1 = split7_out2.add((constant95_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape39_out1 = add42_out1.reshape(concat6_out1);
        let transpose49_out1 = reshape39_out1.permute([0, 2, 1, 3]);
        let constant96_out1 = self.constant96.val();
        let add43_out1 = split7_out3.add((constant96_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape40_out1 = add43_out1.reshape(concat6_out1);
        let transpose50_out1 = reshape40_out1.permute([0, 2, 1, 3]);
        let shape9_out1: [i64; 4] = {
            let axes = &transpose49_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice9_out1: [i64; 1] = shape9_out1[2..3].try_into().unwrap();
        let concat13_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice9_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape41_out1 = transpose49_out1.reshape(concat13_out1);
        let transpose51_out1 = reshape41_out1.permute([0, 2, 1]);
        let reshape42_out1 = transpose51_out1.reshape([1, 16, 64, -1]);
        let mul13_out1 = transpose48_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul14_out1 = reshape42_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul39_out1 = mul13_out1.matmul(mul14_out1);
        let add44_out1 = matmul39_out1.add(where1_out1.clone());
        let softmax7_out1 = burn::tensor::activation::softmax(add44_out1, 3);
        let isnan7_out1 = softmax7_out1.clone().is_nan();
        let where8_out1 = softmax7_out1.mask_fill(isnan7_out1, constant364_out1);
        let matmul40_out1 = where8_out1.matmul(transpose50_out1);
        let transpose52_out1 = matmul40_out1.permute([0, 2, 1, 3]);
        let reshape43_out1 = transpose52_out1.reshape(concat8_out1);
        let linear27_out1 = self.linear27.forward(reshape43_out1);
        let add45_out1 = add40_out1.add(linear27_out1);
        let layernormalization22_out1 = {
            let dtype = add45_out1.clone().dtype();
            self.layernormalization22
                .forward(add45_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear28_out1 = self.linear28.forward(layernormalization22_out1);
        let gelu15_out1 = burn::tensor::activation::gelu(linear28_out1);
        let linear29_out1 = self.linear29.forward(gelu15_out1);
        let add46_out1 = add45_out1.add(linear29_out1);
        let layernormalization23_out1 = {
            let dtype = add46_out1.clone().dtype();
            self.layernormalization23
                .forward(add46_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear30_out1 = self.linear30.forward(layernormalization23_out1);
        let split_tensors = linear30_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split8_out1, split8_out2, split8_out3] = split_tensors.try_into().unwrap();
        let constant107_out1 = self.constant107.val();
        let add47_out1 = split8_out1.add((constant107_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape44_out1 = add47_out1.reshape(concat6_out1);
        let transpose53_out1 = reshape44_out1.permute([0, 2, 1, 3]);
        let constant105_out1 = self.constant105.val();
        let add48_out1 = split8_out2.add((constant105_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape45_out1 = add48_out1.reshape(concat6_out1);
        let transpose54_out1 = reshape45_out1.permute([0, 2, 1, 3]);
        let constant106_out1 = self.constant106.val();
        let add49_out1 = split8_out3.add((constant106_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape46_out1 = add49_out1.reshape(concat6_out1);
        let transpose55_out1 = reshape46_out1.permute([0, 2, 1, 3]);
        let shape10_out1: [i64; 4] = {
            let axes = &transpose54_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice10_out1: [i64; 1] = shape10_out1[2..3].try_into().unwrap();
        let concat14_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice10_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape47_out1 = transpose54_out1.reshape(concat14_out1);
        let transpose56_out1 = reshape47_out1.permute([0, 2, 1]);
        let reshape48_out1 = transpose56_out1.reshape([1, 16, 64, -1]);
        let mul15_out1 = transpose53_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul16_out1 =
            reshape48_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul45_out1 = mul15_out1.matmul(mul16_out1);
        let add50_out1 = matmul45_out1.add(where1_out1);
        let softmax8_out1 = burn::tensor::activation::softmax(add50_out1, 3);
        let isnan8_out1 = softmax8_out1.clone().is_nan();
        let where9_out1 = softmax8_out1.mask_fill(isnan8_out1, constant364_out1);
        let matmul46_out1 = where9_out1.matmul(transpose55_out1);
        let transpose57_out1 = matmul46_out1.permute([0, 2, 1, 3]);
        let reshape49_out1 = transpose57_out1.reshape(concat8_out1);
        let linear31_out1 = self.linear31.forward(reshape49_out1);
        let add51_out1 = add46_out1.add(linear31_out1);
        add51_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule6<B: Backend> {
    layernormalization24: LayerNorm<B>,
    linear32: Linear<B>,
    linear33: Linear<B>,
    layernormalization25: LayerNorm<B>,
    linear34: Linear<B>,
    constant117: burn::module::Param<Tensor<B, 1>>,
    constant115: burn::module::Param<Tensor<B, 1>>,
    constant116: burn::module::Param<Tensor<B, 1>>,
    linear35: Linear<B>,
    layernormalization26: LayerNorm<B>,
    linear36: Linear<B>,
    linear37: Linear<B>,
    layernormalization27: LayerNorm<B>,
    linear38: Linear<B>,
    constant127: burn::module::Param<Tensor<B, 1>>,
    constant125: burn::module::Param<Tensor<B, 1>>,
    constant126: burn::module::Param<Tensor<B, 1>>,
    linear39: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule6<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization24 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear32 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear33 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization25 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear34 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant117: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant115: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant116: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear35 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization26 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear36 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear37 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization27 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear38 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant127: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant125: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant126: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear39 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization24,
            linear32,
            linear33,
            layernormalization25,
            linear34,
            constant117,
            constant115,
            constant116,
            linear35,
            layernormalization26,
            linear36,
            linear37,
            layernormalization27,
            linear38,
            constant127,
            constant125,
            constant126,
            linear39,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add51_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization24_out1 = {
            let dtype = add51_out1.clone().dtype();
            self.layernormalization24
                .forward(add51_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear32_out1 = self.linear32.forward(layernormalization24_out1);
        let gelu16_out1 = burn::tensor::activation::gelu(linear32_out1);
        let linear33_out1 = self.linear33.forward(gelu16_out1);
        let add52_out1 = add51_out1.add(linear33_out1);
        let layernormalization25_out1 = {
            let dtype = add52_out1.clone().dtype();
            self.layernormalization25
                .forward(add52_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear34_out1 = self.linear34.forward(layernormalization25_out1);
        let split_tensors = linear34_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split9_out1, split9_out2, split9_out3] = split_tensors.try_into().unwrap();
        let constant117_out1 = self.constant117.val();
        let add53_out1 = split9_out1.add((constant117_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape50_out1 = add53_out1.reshape(concat6_out1);
        let transpose58_out1 = reshape50_out1.permute([0, 2, 1, 3]);
        let constant115_out1 = self.constant115.val();
        let add54_out1 = split9_out2.add((constant115_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape51_out1 = add54_out1.reshape(concat6_out1);
        let transpose59_out1 = reshape51_out1.permute([0, 2, 1, 3]);
        let constant116_out1 = self.constant116.val();
        let add55_out1 = split9_out3.add((constant116_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape52_out1 = add55_out1.reshape(concat6_out1);
        let transpose60_out1 = reshape52_out1.permute([0, 2, 1, 3]);
        let shape11_out1: [i64; 4] = {
            let axes = &transpose59_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice11_out1: [i64; 1] = shape11_out1[2..3].try_into().unwrap();
        let concat15_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice11_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape53_out1 = transpose59_out1.reshape(concat15_out1);
        let transpose61_out1 = reshape53_out1.permute([0, 2, 1]);
        let reshape54_out1 = transpose61_out1.reshape([1, 16, 64, -1]);
        let mul17_out1 = transpose58_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul18_out1 = reshape54_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul51_out1 = mul17_out1.matmul(mul18_out1);
        let add56_out1 = matmul51_out1.add(where1_out1.clone());
        let softmax9_out1 = burn::tensor::activation::softmax(add56_out1, 3);
        let isnan9_out1 = softmax9_out1.clone().is_nan();
        let where10_out1 = softmax9_out1.mask_fill(isnan9_out1, constant364_out1);
        let matmul52_out1 = where10_out1.matmul(transpose60_out1);
        let transpose62_out1 = matmul52_out1.permute([0, 2, 1, 3]);
        let reshape55_out1 = transpose62_out1.reshape(concat8_out1);
        let linear35_out1 = self.linear35.forward(reshape55_out1);
        let add57_out1 = add52_out1.add(linear35_out1);
        let layernormalization26_out1 = {
            let dtype = add57_out1.clone().dtype();
            self.layernormalization26
                .forward(add57_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear36_out1 = self.linear36.forward(layernormalization26_out1);
        let gelu17_out1 = burn::tensor::activation::gelu(linear36_out1);
        let linear37_out1 = self.linear37.forward(gelu17_out1);
        let add58_out1 = add57_out1.add(linear37_out1);
        let layernormalization27_out1 = {
            let dtype = add58_out1.clone().dtype();
            self.layernormalization27
                .forward(add58_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear38_out1 = self.linear38.forward(layernormalization27_out1);
        let split_tensors = linear38_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split10_out1, split10_out2, split10_out3] = split_tensors.try_into().unwrap();
        let constant127_out1 = self.constant127.val();
        let add59_out1 = split10_out1.add((constant127_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape56_out1 = add59_out1.reshape(concat6_out1);
        let transpose63_out1 = reshape56_out1.permute([0, 2, 1, 3]);
        let constant125_out1 = self.constant125.val();
        let add60_out1 = split10_out2.add((constant125_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape57_out1 = add60_out1.reshape(concat6_out1);
        let transpose64_out1 = reshape57_out1.permute([0, 2, 1, 3]);
        let constant126_out1 = self.constant126.val();
        let add61_out1 = split10_out3.add((constant126_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape58_out1 = add61_out1.reshape(concat6_out1);
        let transpose65_out1 = reshape58_out1.permute([0, 2, 1, 3]);
        let shape12_out1: [i64; 4] = {
            let axes = &transpose64_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice12_out1: [i64; 1] = shape12_out1[2..3].try_into().unwrap();
        let concat16_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice12_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape59_out1 = transpose64_out1.reshape(concat16_out1);
        let transpose66_out1 = reshape59_out1.permute([0, 2, 1]);
        let reshape60_out1 = transpose66_out1.reshape([1, 16, 64, -1]);
        let mul19_out1 = transpose63_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul20_out1 =
            reshape60_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul57_out1 = mul19_out1.matmul(mul20_out1);
        let add62_out1 = matmul57_out1.add(where1_out1);
        let softmax10_out1 = burn::tensor::activation::softmax(add62_out1, 3);
        let isnan10_out1 = softmax10_out1.clone().is_nan();
        let where11_out1 = softmax10_out1.mask_fill(isnan10_out1, constant364_out1);
        let matmul58_out1 = where11_out1.matmul(transpose65_out1);
        let transpose67_out1 = matmul58_out1.permute([0, 2, 1, 3]);
        let reshape61_out1 = transpose67_out1.reshape(concat8_out1);
        let linear39_out1 = self.linear39.forward(reshape61_out1);
        let add63_out1 = add58_out1.add(linear39_out1);
        add63_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule7<B: Backend> {
    layernormalization28: LayerNorm<B>,
    linear40: Linear<B>,
    linear41: Linear<B>,
    layernormalization29: LayerNorm<B>,
    linear42: Linear<B>,
    constant137: burn::module::Param<Tensor<B, 1>>,
    constant135: burn::module::Param<Tensor<B, 1>>,
    constant136: burn::module::Param<Tensor<B, 1>>,
    linear43: Linear<B>,
    layernormalization30: LayerNorm<B>,
    linear44: Linear<B>,
    linear45: Linear<B>,
    layernormalization31: LayerNorm<B>,
    linear46: Linear<B>,
    constant147: burn::module::Param<Tensor<B, 1>>,
    constant145: burn::module::Param<Tensor<B, 1>>,
    constant146: burn::module::Param<Tensor<B, 1>>,
    linear47: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule7<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization28 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear40 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear41 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization29 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear42 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant137: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant135: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant136: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear43 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization30 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear44 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear45 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization31 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear46 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant147: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant145: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant146: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear47 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization28,
            linear40,
            linear41,
            layernormalization29,
            linear42,
            constant137,
            constant135,
            constant136,
            linear43,
            layernormalization30,
            linear44,
            linear45,
            layernormalization31,
            linear46,
            constant147,
            constant145,
            constant146,
            linear47,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add63_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization28_out1 = {
            let dtype = add63_out1.clone().dtype();
            self.layernormalization28
                .forward(add63_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear40_out1 = self.linear40.forward(layernormalization28_out1);
        let gelu18_out1 = burn::tensor::activation::gelu(linear40_out1);
        let linear41_out1 = self.linear41.forward(gelu18_out1);
        let add64_out1 = add63_out1.add(linear41_out1);
        let layernormalization29_out1 = {
            let dtype = add64_out1.clone().dtype();
            self.layernormalization29
                .forward(add64_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear42_out1 = self.linear42.forward(layernormalization29_out1);
        let split_tensors = linear42_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split11_out1, split11_out2, split11_out3] = split_tensors.try_into().unwrap();
        let constant137_out1 = self.constant137.val();
        let add65_out1 = split11_out1.add((constant137_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape62_out1 = add65_out1.reshape(concat6_out1);
        let transpose68_out1 = reshape62_out1.permute([0, 2, 1, 3]);
        let constant135_out1 = self.constant135.val();
        let add66_out1 = split11_out2.add((constant135_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape63_out1 = add66_out1.reshape(concat6_out1);
        let transpose69_out1 = reshape63_out1.permute([0, 2, 1, 3]);
        let constant136_out1 = self.constant136.val();
        let add67_out1 = split11_out3.add((constant136_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape64_out1 = add67_out1.reshape(concat6_out1);
        let transpose70_out1 = reshape64_out1.permute([0, 2, 1, 3]);
        let shape13_out1: [i64; 4] = {
            let axes = &transpose69_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice13_out1: [i64; 1] = shape13_out1[2..3].try_into().unwrap();
        let concat17_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice13_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape65_out1 = transpose69_out1.reshape(concat17_out1);
        let transpose71_out1 = reshape65_out1.permute([0, 2, 1]);
        let reshape66_out1 = transpose71_out1.reshape([1, 16, 64, -1]);
        let mul21_out1 = transpose68_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul22_out1 = reshape66_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul63_out1 = mul21_out1.matmul(mul22_out1);
        let add68_out1 = matmul63_out1.add(where1_out1.clone());
        let softmax11_out1 = burn::tensor::activation::softmax(add68_out1, 3);
        let isnan11_out1 = softmax11_out1.clone().is_nan();
        let where12_out1 = softmax11_out1.mask_fill(isnan11_out1, constant364_out1);
        let matmul64_out1 = where12_out1.matmul(transpose70_out1);
        let transpose72_out1 = matmul64_out1.permute([0, 2, 1, 3]);
        let reshape67_out1 = transpose72_out1.reshape(concat8_out1);
        let linear43_out1 = self.linear43.forward(reshape67_out1);
        let add69_out1 = add64_out1.add(linear43_out1);
        let layernormalization30_out1 = {
            let dtype = add69_out1.clone().dtype();
            self.layernormalization30
                .forward(add69_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear44_out1 = self.linear44.forward(layernormalization30_out1);
        let gelu19_out1 = burn::tensor::activation::gelu(linear44_out1);
        let linear45_out1 = self.linear45.forward(gelu19_out1);
        let add70_out1 = add69_out1.add(linear45_out1);
        let layernormalization31_out1 = {
            let dtype = add70_out1.clone().dtype();
            self.layernormalization31
                .forward(add70_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear46_out1 = self.linear46.forward(layernormalization31_out1);
        let split_tensors = linear46_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split12_out1, split12_out2, split12_out3] = split_tensors.try_into().unwrap();
        let constant147_out1 = self.constant147.val();
        let add71_out1 = split12_out1.add((constant147_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape68_out1 = add71_out1.reshape(concat6_out1);
        let transpose73_out1 = reshape68_out1.permute([0, 2, 1, 3]);
        let constant145_out1 = self.constant145.val();
        let add72_out1 = split12_out2.add((constant145_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape69_out1 = add72_out1.reshape(concat6_out1);
        let transpose74_out1 = reshape69_out1.permute([0, 2, 1, 3]);
        let constant146_out1 = self.constant146.val();
        let add73_out1 = split12_out3.add((constant146_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape70_out1 = add73_out1.reshape(concat6_out1);
        let transpose75_out1 = reshape70_out1.permute([0, 2, 1, 3]);
        let shape14_out1: [i64; 4] = {
            let axes = &transpose74_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice14_out1: [i64; 1] = shape14_out1[2..3].try_into().unwrap();
        let concat18_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice14_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape71_out1 = transpose74_out1.reshape(concat18_out1);
        let transpose76_out1 = reshape71_out1.permute([0, 2, 1]);
        let reshape72_out1 = transpose76_out1.reshape([1, 16, 64, -1]);
        let mul23_out1 = transpose73_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul24_out1 =
            reshape72_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul69_out1 = mul23_out1.matmul(mul24_out1);
        let add74_out1 = matmul69_out1.add(where1_out1);
        let softmax12_out1 = burn::tensor::activation::softmax(add74_out1, 3);
        let isnan12_out1 = softmax12_out1.clone().is_nan();
        let where13_out1 = softmax12_out1.mask_fill(isnan12_out1, constant364_out1);
        let matmul70_out1 = where13_out1.matmul(transpose75_out1);
        let transpose77_out1 = matmul70_out1.permute([0, 2, 1, 3]);
        let reshape73_out1 = transpose77_out1.reshape(concat8_out1);
        let linear47_out1 = self.linear47.forward(reshape73_out1);
        let add75_out1 = add70_out1.add(linear47_out1);
        add75_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule8<B: Backend> {
    layernormalization32: LayerNorm<B>,
    linear48: Linear<B>,
    linear49: Linear<B>,
    layernormalization33: LayerNorm<B>,
    linear50: Linear<B>,
    constant157: burn::module::Param<Tensor<B, 1>>,
    constant155: burn::module::Param<Tensor<B, 1>>,
    constant156: burn::module::Param<Tensor<B, 1>>,
    linear51: Linear<B>,
    layernormalization34: LayerNorm<B>,
    linear52: Linear<B>,
    linear53: Linear<B>,
    layernormalization35: LayerNorm<B>,
    linear54: Linear<B>,
    constant167: burn::module::Param<Tensor<B, 1>>,
    constant165: burn::module::Param<Tensor<B, 1>>,
    constant166: burn::module::Param<Tensor<B, 1>>,
    linear55: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule8<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization32 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear48 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear49 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization33 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear50 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant157: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant155: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant156: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear51 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization34 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear52 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear53 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization35 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear54 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant167: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant165: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant166: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear55 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization32,
            linear48,
            linear49,
            layernormalization33,
            linear50,
            constant157,
            constant155,
            constant156,
            linear51,
            layernormalization34,
            linear52,
            linear53,
            layernormalization35,
            linear54,
            constant167,
            constant165,
            constant166,
            linear55,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add75_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization32_out1 = {
            let dtype = add75_out1.clone().dtype();
            self.layernormalization32
                .forward(add75_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear48_out1 = self.linear48.forward(layernormalization32_out1);
        let gelu20_out1 = burn::tensor::activation::gelu(linear48_out1);
        let linear49_out1 = self.linear49.forward(gelu20_out1);
        let add76_out1 = add75_out1.add(linear49_out1);
        let layernormalization33_out1 = {
            let dtype = add76_out1.clone().dtype();
            self.layernormalization33
                .forward(add76_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear50_out1 = self.linear50.forward(layernormalization33_out1);
        let split_tensors = linear50_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split13_out1, split13_out2, split13_out3] = split_tensors.try_into().unwrap();
        let constant157_out1 = self.constant157.val();
        let add77_out1 = split13_out1.add((constant157_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape74_out1 = add77_out1.reshape(concat6_out1);
        let transpose78_out1 = reshape74_out1.permute([0, 2, 1, 3]);
        let constant155_out1 = self.constant155.val();
        let add78_out1 = split13_out2.add((constant155_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape75_out1 = add78_out1.reshape(concat6_out1);
        let transpose79_out1 = reshape75_out1.permute([0, 2, 1, 3]);
        let constant156_out1 = self.constant156.val();
        let add79_out1 = split13_out3.add((constant156_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape76_out1 = add79_out1.reshape(concat6_out1);
        let transpose80_out1 = reshape76_out1.permute([0, 2, 1, 3]);
        let shape15_out1: [i64; 4] = {
            let axes = &transpose79_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice15_out1: [i64; 1] = shape15_out1[2..3].try_into().unwrap();
        let concat19_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice15_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape77_out1 = transpose79_out1.reshape(concat19_out1);
        let transpose81_out1 = reshape77_out1.permute([0, 2, 1]);
        let reshape78_out1 = transpose81_out1.reshape([1, 16, 64, -1]);
        let mul25_out1 = transpose78_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul26_out1 = reshape78_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul75_out1 = mul25_out1.matmul(mul26_out1);
        let add80_out1 = matmul75_out1.add(where1_out1.clone());
        let softmax13_out1 = burn::tensor::activation::softmax(add80_out1, 3);
        let isnan13_out1 = softmax13_out1.clone().is_nan();
        let where14_out1 = softmax13_out1.mask_fill(isnan13_out1, constant364_out1);
        let matmul76_out1 = where14_out1.matmul(transpose80_out1);
        let transpose82_out1 = matmul76_out1.permute([0, 2, 1, 3]);
        let reshape79_out1 = transpose82_out1.reshape(concat8_out1);
        let linear51_out1 = self.linear51.forward(reshape79_out1);
        let add81_out1 = add76_out1.add(linear51_out1);
        let layernormalization34_out1 = {
            let dtype = add81_out1.clone().dtype();
            self.layernormalization34
                .forward(add81_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear52_out1 = self.linear52.forward(layernormalization34_out1);
        let gelu21_out1 = burn::tensor::activation::gelu(linear52_out1);
        let linear53_out1 = self.linear53.forward(gelu21_out1);
        let add82_out1 = add81_out1.add(linear53_out1);
        let layernormalization35_out1 = {
            let dtype = add82_out1.clone().dtype();
            self.layernormalization35
                .forward(add82_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear54_out1 = self.linear54.forward(layernormalization35_out1);
        let split_tensors = linear54_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split14_out1, split14_out2, split14_out3] = split_tensors.try_into().unwrap();
        let constant167_out1 = self.constant167.val();
        let add83_out1 = split14_out1.add((constant167_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape80_out1 = add83_out1.reshape(concat6_out1);
        let transpose83_out1 = reshape80_out1.permute([0, 2, 1, 3]);
        let constant165_out1 = self.constant165.val();
        let add84_out1 = split14_out2.add((constant165_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape81_out1 = add84_out1.reshape(concat6_out1);
        let transpose84_out1 = reshape81_out1.permute([0, 2, 1, 3]);
        let constant166_out1 = self.constant166.val();
        let add85_out1 = split14_out3.add((constant166_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape82_out1 = add85_out1.reshape(concat6_out1);
        let transpose85_out1 = reshape82_out1.permute([0, 2, 1, 3]);
        let shape16_out1: [i64; 4] = {
            let axes = &transpose84_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice16_out1: [i64; 1] = shape16_out1[2..3].try_into().unwrap();
        let concat20_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice16_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape83_out1 = transpose84_out1.reshape(concat20_out1);
        let transpose86_out1 = reshape83_out1.permute([0, 2, 1]);
        let reshape84_out1 = transpose86_out1.reshape([1, 16, 64, -1]);
        let mul27_out1 = transpose83_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul28_out1 =
            reshape84_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul81_out1 = mul27_out1.matmul(mul28_out1);
        let add86_out1 = matmul81_out1.add(where1_out1);
        let softmax14_out1 = burn::tensor::activation::softmax(add86_out1, 3);
        let isnan14_out1 = softmax14_out1.clone().is_nan();
        let where15_out1 = softmax14_out1.mask_fill(isnan14_out1, constant364_out1);
        let matmul82_out1 = where15_out1.matmul(transpose85_out1);
        let transpose87_out1 = matmul82_out1.permute([0, 2, 1, 3]);
        let reshape85_out1 = transpose87_out1.reshape(concat8_out1);
        let linear55_out1 = self.linear55.forward(reshape85_out1);
        let add87_out1 = add82_out1.add(linear55_out1);
        add87_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule9<B: Backend> {
    layernormalization36: LayerNorm<B>,
    linear56: Linear<B>,
    linear57: Linear<B>,
    layernormalization37: LayerNorm<B>,
    linear58: Linear<B>,
    constant177: burn::module::Param<Tensor<B, 1>>,
    constant175: burn::module::Param<Tensor<B, 1>>,
    constant176: burn::module::Param<Tensor<B, 1>>,
    linear59: Linear<B>,
    layernormalization38: LayerNorm<B>,
    linear60: Linear<B>,
    linear61: Linear<B>,
    layernormalization39: LayerNorm<B>,
    linear62: Linear<B>,
    constant187: burn::module::Param<Tensor<B, 1>>,
    constant185: burn::module::Param<Tensor<B, 1>>,
    constant186: burn::module::Param<Tensor<B, 1>>,
    linear63: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule9<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization36 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear56 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear57 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization37 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear58 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant177: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant175: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant176: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear59 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization38 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear60 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear61 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization39 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear62 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant187: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant185: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant186: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear63 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization36,
            linear56,
            linear57,
            layernormalization37,
            linear58,
            constant177,
            constant175,
            constant176,
            linear59,
            layernormalization38,
            linear60,
            linear61,
            layernormalization39,
            linear62,
            constant187,
            constant185,
            constant186,
            linear63,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add87_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization36_out1 = {
            let dtype = add87_out1.clone().dtype();
            self.layernormalization36
                .forward(add87_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear56_out1 = self.linear56.forward(layernormalization36_out1);
        let gelu22_out1 = burn::tensor::activation::gelu(linear56_out1);
        let linear57_out1 = self.linear57.forward(gelu22_out1);
        let add88_out1 = add87_out1.add(linear57_out1);
        let layernormalization37_out1 = {
            let dtype = add88_out1.clone().dtype();
            self.layernormalization37
                .forward(add88_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear58_out1 = self.linear58.forward(layernormalization37_out1);
        let split_tensors = linear58_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split15_out1, split15_out2, split15_out3] = split_tensors.try_into().unwrap();
        let constant177_out1 = self.constant177.val();
        let add89_out1 = split15_out1.add((constant177_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape86_out1 = add89_out1.reshape(concat6_out1);
        let transpose88_out1 = reshape86_out1.permute([0, 2, 1, 3]);
        let constant175_out1 = self.constant175.val();
        let add90_out1 = split15_out2.add((constant175_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape87_out1 = add90_out1.reshape(concat6_out1);
        let transpose89_out1 = reshape87_out1.permute([0, 2, 1, 3]);
        let constant176_out1 = self.constant176.val();
        let add91_out1 = split15_out3.add((constant176_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape88_out1 = add91_out1.reshape(concat6_out1);
        let transpose90_out1 = reshape88_out1.permute([0, 2, 1, 3]);
        let shape17_out1: [i64; 4] = {
            let axes = &transpose89_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice17_out1: [i64; 1] = shape17_out1[2..3].try_into().unwrap();
        let concat21_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice17_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape89_out1 = transpose89_out1.reshape(concat21_out1);
        let transpose91_out1 = reshape89_out1.permute([0, 2, 1]);
        let reshape90_out1 = transpose91_out1.reshape([1, 16, 64, -1]);
        let mul29_out1 = transpose88_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul30_out1 = reshape90_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul87_out1 = mul29_out1.matmul(mul30_out1);
        let add92_out1 = matmul87_out1.add(where1_out1.clone());
        let softmax15_out1 = burn::tensor::activation::softmax(add92_out1, 3);
        let isnan15_out1 = softmax15_out1.clone().is_nan();
        let where16_out1 = softmax15_out1.mask_fill(isnan15_out1, constant364_out1);
        let matmul88_out1 = where16_out1.matmul(transpose90_out1);
        let transpose92_out1 = matmul88_out1.permute([0, 2, 1, 3]);
        let reshape91_out1 = transpose92_out1.reshape(concat8_out1);
        let linear59_out1 = self.linear59.forward(reshape91_out1);
        let add93_out1 = add88_out1.add(linear59_out1);
        let layernormalization38_out1 = {
            let dtype = add93_out1.clone().dtype();
            self.layernormalization38
                .forward(add93_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear60_out1 = self.linear60.forward(layernormalization38_out1);
        let gelu23_out1 = burn::tensor::activation::gelu(linear60_out1);
        let linear61_out1 = self.linear61.forward(gelu23_out1);
        let add94_out1 = add93_out1.add(linear61_out1);
        let layernormalization39_out1 = {
            let dtype = add94_out1.clone().dtype();
            self.layernormalization39
                .forward(add94_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear62_out1 = self.linear62.forward(layernormalization39_out1);
        let split_tensors = linear62_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split16_out1, split16_out2, split16_out3] = split_tensors.try_into().unwrap();
        let constant187_out1 = self.constant187.val();
        let add95_out1 = split16_out1.add((constant187_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape92_out1 = add95_out1.reshape(concat6_out1);
        let transpose93_out1 = reshape92_out1.permute([0, 2, 1, 3]);
        let constant185_out1 = self.constant185.val();
        let add96_out1 = split16_out2.add((constant185_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape93_out1 = add96_out1.reshape(concat6_out1);
        let transpose94_out1 = reshape93_out1.permute([0, 2, 1, 3]);
        let constant186_out1 = self.constant186.val();
        let add97_out1 = split16_out3.add((constant186_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape94_out1 = add97_out1.reshape(concat6_out1);
        let transpose95_out1 = reshape94_out1.permute([0, 2, 1, 3]);
        let shape18_out1: [i64; 4] = {
            let axes = &transpose94_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice18_out1: [i64; 1] = shape18_out1[2..3].try_into().unwrap();
        let concat22_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice18_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape95_out1 = transpose94_out1.reshape(concat22_out1);
        let transpose96_out1 = reshape95_out1.permute([0, 2, 1]);
        let reshape96_out1 = transpose96_out1.reshape([1, 16, 64, -1]);
        let mul31_out1 = transpose93_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul32_out1 =
            reshape96_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul93_out1 = mul31_out1.matmul(mul32_out1);
        let add98_out1 = matmul93_out1.add(where1_out1);
        let softmax16_out1 = burn::tensor::activation::softmax(add98_out1, 3);
        let isnan16_out1 = softmax16_out1.clone().is_nan();
        let where17_out1 = softmax16_out1.mask_fill(isnan16_out1, constant364_out1);
        let matmul94_out1 = where17_out1.matmul(transpose95_out1);
        let transpose97_out1 = matmul94_out1.permute([0, 2, 1, 3]);
        let reshape97_out1 = transpose97_out1.reshape(concat8_out1);
        let linear63_out1 = self.linear63.forward(reshape97_out1);
        let add99_out1 = add94_out1.add(linear63_out1);
        add99_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule10<B: Backend> {
    layernormalization40: LayerNorm<B>,
    linear64: Linear<B>,
    linear65: Linear<B>,
    layernormalization41: LayerNorm<B>,
    linear66: Linear<B>,
    constant197: burn::module::Param<Tensor<B, 1>>,
    constant195: burn::module::Param<Tensor<B, 1>>,
    constant196: burn::module::Param<Tensor<B, 1>>,
    linear67: Linear<B>,
    layernormalization42: LayerNorm<B>,
    linear68: Linear<B>,
    linear69: Linear<B>,
    layernormalization43: LayerNorm<B>,
    linear70: Linear<B>,
    constant207: burn::module::Param<Tensor<B, 1>>,
    constant205: burn::module::Param<Tensor<B, 1>>,
    constant206: burn::module::Param<Tensor<B, 1>>,
    linear71: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule10<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization40 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear64 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear65 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization41 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear66 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant197: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant195: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant196: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear67 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization42 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear68 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear69 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization43 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear70 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant207: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant205: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant206: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear71 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization40,
            linear64,
            linear65,
            layernormalization41,
            linear66,
            constant197,
            constant195,
            constant196,
            linear67,
            layernormalization42,
            linear68,
            linear69,
            layernormalization43,
            linear70,
            constant207,
            constant205,
            constant206,
            linear71,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add99_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization40_out1 = {
            let dtype = add99_out1.clone().dtype();
            self.layernormalization40
                .forward(add99_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear64_out1 = self.linear64.forward(layernormalization40_out1);
        let gelu24_out1 = burn::tensor::activation::gelu(linear64_out1);
        let linear65_out1 = self.linear65.forward(gelu24_out1);
        let add100_out1 = add99_out1.add(linear65_out1);
        let layernormalization41_out1 = {
            let dtype = add100_out1.clone().dtype();
            self.layernormalization41
                .forward(add100_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear66_out1 = self.linear66.forward(layernormalization41_out1);
        let split_tensors = linear66_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split17_out1, split17_out2, split17_out3] = split_tensors.try_into().unwrap();
        let constant197_out1 = self.constant197.val();
        let add101_out1 = split17_out1.add((constant197_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape98_out1 = add101_out1.reshape(concat6_out1);
        let transpose98_out1 = reshape98_out1.permute([0, 2, 1, 3]);
        let constant195_out1 = self.constant195.val();
        let add102_out1 = split17_out2.add((constant195_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape99_out1 = add102_out1.reshape(concat6_out1);
        let transpose99_out1 = reshape99_out1.permute([0, 2, 1, 3]);
        let constant196_out1 = self.constant196.val();
        let add103_out1 = split17_out3.add((constant196_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape100_out1 = add103_out1.reshape(concat6_out1);
        let transpose100_out1 = reshape100_out1.permute([0, 2, 1, 3]);
        let shape19_out1: [i64; 4] = {
            let axes = &transpose99_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice19_out1: [i64; 1] = shape19_out1[2..3].try_into().unwrap();
        let concat23_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice19_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape101_out1 = transpose99_out1.reshape(concat23_out1);
        let transpose101_out1 = reshape101_out1.permute([0, 2, 1]);
        let reshape102_out1 = transpose101_out1.reshape([1, 16, 64, -1]);
        let mul33_out1 = transpose98_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul34_out1 = reshape102_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul99_out1 = mul33_out1.matmul(mul34_out1);
        let add104_out1 = matmul99_out1.add(where1_out1.clone());
        let softmax17_out1 = burn::tensor::activation::softmax(add104_out1, 3);
        let isnan17_out1 = softmax17_out1.clone().is_nan();
        let where18_out1 = softmax17_out1.mask_fill(isnan17_out1, constant364_out1);
        let matmul100_out1 = where18_out1.matmul(transpose100_out1);
        let transpose102_out1 = matmul100_out1.permute([0, 2, 1, 3]);
        let reshape103_out1 = transpose102_out1.reshape(concat8_out1);
        let linear67_out1 = self.linear67.forward(reshape103_out1);
        let add105_out1 = add100_out1.add(linear67_out1);
        let layernormalization42_out1 = {
            let dtype = add105_out1.clone().dtype();
            self.layernormalization42
                .forward(add105_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear68_out1 = self.linear68.forward(layernormalization42_out1);
        let gelu25_out1 = burn::tensor::activation::gelu(linear68_out1);
        let linear69_out1 = self.linear69.forward(gelu25_out1);
        let add106_out1 = add105_out1.add(linear69_out1);
        let layernormalization43_out1 = {
            let dtype = add106_out1.clone().dtype();
            self.layernormalization43
                .forward(add106_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear70_out1 = self.linear70.forward(layernormalization43_out1);
        let split_tensors = linear70_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split18_out1, split18_out2, split18_out3] = split_tensors.try_into().unwrap();
        let constant207_out1 = self.constant207.val();
        let add107_out1 = split18_out1.add((constant207_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape104_out1 = add107_out1.reshape(concat6_out1);
        let transpose103_out1 = reshape104_out1.permute([0, 2, 1, 3]);
        let constant205_out1 = self.constant205.val();
        let add108_out1 = split18_out2.add((constant205_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape105_out1 = add108_out1.reshape(concat6_out1);
        let transpose104_out1 = reshape105_out1.permute([0, 2, 1, 3]);
        let constant206_out1 = self.constant206.val();
        let add109_out1 = split18_out3.add((constant206_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape106_out1 = add109_out1.reshape(concat6_out1);
        let transpose105_out1 = reshape106_out1.permute([0, 2, 1, 3]);
        let shape20_out1: [i64; 4] = {
            let axes = &transpose104_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice20_out1: [i64; 1] = shape20_out1[2..3].try_into().unwrap();
        let concat24_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice20_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape107_out1 = transpose104_out1.reshape(concat24_out1);
        let transpose106_out1 = reshape107_out1.permute([0, 2, 1]);
        let reshape108_out1 = transpose106_out1.reshape([1, 16, 64, -1]);
        let mul35_out1 = transpose103_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul36_out1 =
            reshape108_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul105_out1 = mul35_out1.matmul(mul36_out1);
        let add110_out1 = matmul105_out1.add(where1_out1);
        let softmax18_out1 = burn::tensor::activation::softmax(add110_out1, 3);
        let isnan18_out1 = softmax18_out1.clone().is_nan();
        let where19_out1 = softmax18_out1.mask_fill(isnan18_out1, constant364_out1);
        let matmul106_out1 = where19_out1.matmul(transpose105_out1);
        let transpose107_out1 = matmul106_out1.permute([0, 2, 1, 3]);
        let reshape109_out1 = transpose107_out1.reshape(concat8_out1);
        let linear71_out1 = self.linear71.forward(reshape109_out1);
        let add111_out1 = add106_out1.add(linear71_out1);
        add111_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule11<B: Backend> {
    layernormalization44: LayerNorm<B>,
    linear72: Linear<B>,
    linear73: Linear<B>,
    layernormalization45: LayerNorm<B>,
    linear74: Linear<B>,
    constant217: burn::module::Param<Tensor<B, 1>>,
    constant215: burn::module::Param<Tensor<B, 1>>,
    constant216: burn::module::Param<Tensor<B, 1>>,
    linear75: Linear<B>,
    layernormalization46: LayerNorm<B>,
    linear76: Linear<B>,
    linear77: Linear<B>,
    layernormalization47: LayerNorm<B>,
    linear78: Linear<B>,
    constant227: burn::module::Param<Tensor<B, 1>>,
    constant225: burn::module::Param<Tensor<B, 1>>,
    constant226: burn::module::Param<Tensor<B, 1>>,
    linear79: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule11<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization44 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear72 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear73 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization45 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear74 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant217: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant215: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant216: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear75 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization46 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear76 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear77 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization47 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear78 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant227: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant225: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant226: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear79 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization44,
            linear72,
            linear73,
            layernormalization45,
            linear74,
            constant217,
            constant215,
            constant216,
            linear75,
            layernormalization46,
            linear76,
            linear77,
            layernormalization47,
            linear78,
            constant227,
            constant225,
            constant226,
            linear79,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add111_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization44_out1 = {
            let dtype = add111_out1.clone().dtype();
            self.layernormalization44
                .forward(add111_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear72_out1 = self.linear72.forward(layernormalization44_out1);
        let gelu26_out1 = burn::tensor::activation::gelu(linear72_out1);
        let linear73_out1 = self.linear73.forward(gelu26_out1);
        let add112_out1 = add111_out1.add(linear73_out1);
        let layernormalization45_out1 = {
            let dtype = add112_out1.clone().dtype();
            self.layernormalization45
                .forward(add112_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear74_out1 = self.linear74.forward(layernormalization45_out1);
        let split_tensors = linear74_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split19_out1, split19_out2, split19_out3] = split_tensors.try_into().unwrap();
        let constant217_out1 = self.constant217.val();
        let add113_out1 = split19_out1.add((constant217_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape110_out1 = add113_out1.reshape(concat6_out1);
        let transpose108_out1 = reshape110_out1.permute([0, 2, 1, 3]);
        let constant215_out1 = self.constant215.val();
        let add114_out1 = split19_out2.add((constant215_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape111_out1 = add114_out1.reshape(concat6_out1);
        let transpose109_out1 = reshape111_out1.permute([0, 2, 1, 3]);
        let constant216_out1 = self.constant216.val();
        let add115_out1 = split19_out3.add((constant216_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape112_out1 = add115_out1.reshape(concat6_out1);
        let transpose110_out1 = reshape112_out1.permute([0, 2, 1, 3]);
        let shape21_out1: [i64; 4] = {
            let axes = &transpose109_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice21_out1: [i64; 1] = shape21_out1[2..3].try_into().unwrap();
        let concat25_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice21_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape113_out1 = transpose109_out1.reshape(concat25_out1);
        let transpose111_out1 = reshape113_out1.permute([0, 2, 1]);
        let reshape114_out1 = transpose111_out1.reshape([1, 16, 64, -1]);
        let mul37_out1 = transpose108_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul38_out1 = reshape114_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul111_out1 = mul37_out1.matmul(mul38_out1);
        let add116_out1 = matmul111_out1.add(where1_out1.clone());
        let softmax19_out1 = burn::tensor::activation::softmax(add116_out1, 3);
        let isnan19_out1 = softmax19_out1.clone().is_nan();
        let where20_out1 = softmax19_out1.mask_fill(isnan19_out1, constant364_out1);
        let matmul112_out1 = where20_out1.matmul(transpose110_out1);
        let transpose112_out1 = matmul112_out1.permute([0, 2, 1, 3]);
        let reshape115_out1 = transpose112_out1.reshape(concat8_out1);
        let linear75_out1 = self.linear75.forward(reshape115_out1);
        let add117_out1 = add112_out1.add(linear75_out1);
        let layernormalization46_out1 = {
            let dtype = add117_out1.clone().dtype();
            self.layernormalization46
                .forward(add117_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear76_out1 = self.linear76.forward(layernormalization46_out1);
        let gelu27_out1 = burn::tensor::activation::gelu(linear76_out1);
        let linear77_out1 = self.linear77.forward(gelu27_out1);
        let add118_out1 = add117_out1.add(linear77_out1);
        let layernormalization47_out1 = {
            let dtype = add118_out1.clone().dtype();
            self.layernormalization47
                .forward(add118_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear78_out1 = self.linear78.forward(layernormalization47_out1);
        let split_tensors = linear78_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split20_out1, split20_out2, split20_out3] = split_tensors.try_into().unwrap();
        let constant227_out1 = self.constant227.val();
        let add119_out1 = split20_out1.add((constant227_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape116_out1 = add119_out1.reshape(concat6_out1);
        let transpose113_out1 = reshape116_out1.permute([0, 2, 1, 3]);
        let constant225_out1 = self.constant225.val();
        let add120_out1 = split20_out2.add((constant225_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape117_out1 = add120_out1.reshape(concat6_out1);
        let transpose114_out1 = reshape117_out1.permute([0, 2, 1, 3]);
        let constant226_out1 = self.constant226.val();
        let add121_out1 = split20_out3.add((constant226_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape118_out1 = add121_out1.reshape(concat6_out1);
        let transpose115_out1 = reshape118_out1.permute([0, 2, 1, 3]);
        let shape22_out1: [i64; 4] = {
            let axes = &transpose114_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice22_out1: [i64; 1] = shape22_out1[2..3].try_into().unwrap();
        let concat26_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice22_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape119_out1 = transpose114_out1.reshape(concat26_out1);
        let transpose116_out1 = reshape119_out1.permute([0, 2, 1]);
        let reshape120_out1 = transpose116_out1.reshape([1, 16, 64, -1]);
        let mul39_out1 = transpose113_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul40_out1 =
            reshape120_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul117_out1 = mul39_out1.matmul(mul40_out1);
        let add122_out1 = matmul117_out1.add(where1_out1);
        let softmax20_out1 = burn::tensor::activation::softmax(add122_out1, 3);
        let isnan20_out1 = softmax20_out1.clone().is_nan();
        let where21_out1 = softmax20_out1.mask_fill(isnan20_out1, constant364_out1);
        let matmul118_out1 = where21_out1.matmul(transpose115_out1);
        let transpose117_out1 = matmul118_out1.permute([0, 2, 1, 3]);
        let reshape121_out1 = transpose117_out1.reshape(concat8_out1);
        let linear79_out1 = self.linear79.forward(reshape121_out1);
        let add123_out1 = add118_out1.add(linear79_out1);
        add123_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule12<B: Backend> {
    layernormalization48: LayerNorm<B>,
    linear80: Linear<B>,
    linear81: Linear<B>,
    layernormalization49: LayerNorm<B>,
    linear82: Linear<B>,
    constant237: burn::module::Param<Tensor<B, 1>>,
    constant235: burn::module::Param<Tensor<B, 1>>,
    constant236: burn::module::Param<Tensor<B, 1>>,
    linear83: Linear<B>,
    layernormalization50: LayerNorm<B>,
    linear84: Linear<B>,
    linear85: Linear<B>,
    layernormalization51: LayerNorm<B>,
    linear86: Linear<B>,
    constant247: burn::module::Param<Tensor<B, 1>>,
    constant245: burn::module::Param<Tensor<B, 1>>,
    constant246: burn::module::Param<Tensor<B, 1>>,
    linear87: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule12<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization48 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear80 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear81 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization49 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear82 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant237: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant235: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant236: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear83 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization50 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear84 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear85 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization51 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear86 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant247: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant245: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant246: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear87 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        Self {
            layernormalization48,
            linear80,
            linear81,
            layernormalization49,
            linear82,
            constant237,
            constant235,
            constant236,
            linear83,
            layernormalization50,
            linear84,
            linear85,
            layernormalization51,
            linear86,
            constant247,
            constant245,
            constant246,
            linear87,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add123_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization48_out1 = {
            let dtype = add123_out1.clone().dtype();
            self.layernormalization48
                .forward(add123_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear80_out1 = self.linear80.forward(layernormalization48_out1);
        let gelu28_out1 = burn::tensor::activation::gelu(linear80_out1);
        let linear81_out1 = self.linear81.forward(gelu28_out1);
        let add124_out1 = add123_out1.add(linear81_out1);
        let layernormalization49_out1 = {
            let dtype = add124_out1.clone().dtype();
            self.layernormalization49
                .forward(add124_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear82_out1 = self.linear82.forward(layernormalization49_out1);
        let split_tensors = linear82_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split21_out1, split21_out2, split21_out3] = split_tensors.try_into().unwrap();
        let constant237_out1 = self.constant237.val();
        let add125_out1 = split21_out1.add((constant237_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape122_out1 = add125_out1.reshape(concat6_out1);
        let transpose118_out1 = reshape122_out1.permute([0, 2, 1, 3]);
        let constant235_out1 = self.constant235.val();
        let add126_out1 = split21_out2.add((constant235_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape123_out1 = add126_out1.reshape(concat6_out1);
        let transpose119_out1 = reshape123_out1.permute([0, 2, 1, 3]);
        let constant236_out1 = self.constant236.val();
        let add127_out1 = split21_out3.add((constant236_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape124_out1 = add127_out1.reshape(concat6_out1);
        let transpose120_out1 = reshape124_out1.permute([0, 2, 1, 3]);
        let shape23_out1: [i64; 4] = {
            let axes = &transpose119_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice23_out1: [i64; 1] = shape23_out1[2..3].try_into().unwrap();
        let concat27_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice23_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape125_out1 = transpose119_out1.reshape(concat27_out1);
        let transpose121_out1 = reshape125_out1.permute([0, 2, 1]);
        let reshape126_out1 = transpose121_out1.reshape([1, 16, 64, -1]);
        let mul41_out1 = transpose118_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul42_out1 = reshape126_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul123_out1 = mul41_out1.matmul(mul42_out1);
        let add128_out1 = matmul123_out1.add(where1_out1.clone());
        let softmax21_out1 = burn::tensor::activation::softmax(add128_out1, 3);
        let isnan21_out1 = softmax21_out1.clone().is_nan();
        let where22_out1 = softmax21_out1.mask_fill(isnan21_out1, constant364_out1);
        let matmul124_out1 = where22_out1.matmul(transpose120_out1);
        let transpose122_out1 = matmul124_out1.permute([0, 2, 1, 3]);
        let reshape127_out1 = transpose122_out1.reshape(concat8_out1);
        let linear83_out1 = self.linear83.forward(reshape127_out1);
        let add129_out1 = add124_out1.add(linear83_out1);
        let layernormalization50_out1 = {
            let dtype = add129_out1.clone().dtype();
            self.layernormalization50
                .forward(add129_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear84_out1 = self.linear84.forward(layernormalization50_out1);
        let gelu29_out1 = burn::tensor::activation::gelu(linear84_out1);
        let linear85_out1 = self.linear85.forward(gelu29_out1);
        let add130_out1 = add129_out1.add(linear85_out1);
        let layernormalization51_out1 = {
            let dtype = add130_out1.clone().dtype();
            self.layernormalization51
                .forward(add130_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear86_out1 = self.linear86.forward(layernormalization51_out1);
        let split_tensors = linear86_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split22_out1, split22_out2, split22_out3] = split_tensors.try_into().unwrap();
        let constant247_out1 = self.constant247.val();
        let add131_out1 = split22_out1.add((constant247_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape128_out1 = add131_out1.reshape(concat6_out1);
        let transpose123_out1 = reshape128_out1.permute([0, 2, 1, 3]);
        let constant245_out1 = self.constant245.val();
        let add132_out1 = split22_out2.add((constant245_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape129_out1 = add132_out1.reshape(concat6_out1);
        let transpose124_out1 = reshape129_out1.permute([0, 2, 1, 3]);
        let constant246_out1 = self.constant246.val();
        let add133_out1 = split22_out3.add((constant246_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape130_out1 = add133_out1.reshape(concat6_out1);
        let transpose125_out1 = reshape130_out1.permute([0, 2, 1, 3]);
        let shape24_out1: [i64; 4] = {
            let axes = &transpose124_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice24_out1: [i64; 1] = shape24_out1[2..3].try_into().unwrap();
        let concat28_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice24_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape131_out1 = transpose124_out1.reshape(concat28_out1);
        let transpose126_out1 = reshape131_out1.permute([0, 2, 1]);
        let reshape132_out1 = transpose126_out1.reshape([1, 16, 64, -1]);
        let mul43_out1 = transpose123_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul44_out1 =
            reshape132_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul129_out1 = mul43_out1.matmul(mul44_out1);
        let add134_out1 = matmul129_out1.add(where1_out1);
        let softmax22_out1 = burn::tensor::activation::softmax(add134_out1, 3);
        let isnan22_out1 = softmax22_out1.clone().is_nan();
        let where23_out1 = softmax22_out1.mask_fill(isnan22_out1, constant364_out1);
        let matmul130_out1 = where23_out1.matmul(transpose125_out1);
        let transpose127_out1 = matmul130_out1.permute([0, 2, 1, 3]);
        let reshape133_out1 = transpose127_out1.reshape(concat8_out1);
        let linear87_out1 = self.linear87.forward(reshape133_out1);
        let add135_out1 = add130_out1.add(linear87_out1);
        add135_out1
    }
}
#[derive(Module, Debug)]
pub struct Submodule13<B: Backend> {
    layernormalization52: LayerNorm<B>,
    linear88: Linear<B>,
    linear89: Linear<B>,
    layernormalization53: LayerNorm<B>,
    linear90: Linear<B>,
    constant257: burn::module::Param<Tensor<B, 1>>,
    constant255: burn::module::Param<Tensor<B, 1>>,
    constant256: burn::module::Param<Tensor<B, 1>>,
    linear91: Linear<B>,
    layernormalization54: LayerNorm<B>,
    linear92: Linear<B>,
    linear93: Linear<B>,
    layernormalization55: LayerNorm<B>,
    linear94: Linear<B>,
    constant267: burn::module::Param<Tensor<B, 1>>,
    constant265: burn::module::Param<Tensor<B, 1>>,
    constant266: burn::module::Param<Tensor<B, 1>>,
    linear95: Linear<B>,
    layernormalization56: LayerNorm<B>,
    linear96: Linear<B>,
    linear97: Linear<B>,
    layernormalization57: LayerNorm<B>,
    linear98: Linear<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}
impl<B: Backend> Submodule13<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let layernormalization52 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear88 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear89 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization53 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear90 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant257: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant255: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant256: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear91 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization54 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear92 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear93 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization55 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear94 = LinearConfig::new(1024, 3072).with_bias(false).init(device);
        let constant267: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant265: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let constant266: burn::module::Param<Tensor<B, 1>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| {
                Tensor::<B, 1>::zeros([1024], (device, burn::tensor::DType::F32))
            },
            device.clone(),
            false,
            [1024].into(),
        );
        let linear95 = LinearConfig::new(1024, 1024).with_bias(true).init(device);
        let layernormalization56 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear96 = LinearConfig::new(1024, 4096).with_bias(true).init(device);
        let linear97 = LinearConfig::new(4096, 1024).with_bias(true).init(device);
        let layernormalization57 = LayerNormConfig::new(1024)
            .with_epsilon(0.000009999999747378752f64)
            .with_bias(true)
            .init(device);
        let linear98 = LinearConfig::new(1024, 311).with_bias(true).init(device);
        Self {
            layernormalization52,
            linear88,
            linear89,
            layernormalization53,
            linear90,
            constant257,
            constant255,
            constant256,
            linear91,
            layernormalization54,
            linear92,
            linear93,
            layernormalization55,
            linear94,
            constant267,
            constant265,
            constant266,
            linear95,
            layernormalization56,
            linear96,
            linear97,
            layernormalization57,
            linear98,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }
    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(
        &self,
        add135_out1: Tensor<B, 3>,
        concat6_out1: [i64; 4],
        constant280_out1: [i64; 1],
        constant361_out1: [i64; 1],
        constant282_out1: Tensor<B, 1>,
        where1_out1: Tensor<B, 4>,
        constant364_out1: f32,
        concat8_out1: [i64; 3],
    ) -> Tensor<B, 3> {
        let layernormalization52_out1 = {
            let dtype = add135_out1.clone().dtype();
            self.layernormalization52
                .forward(add135_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear88_out1 = self.linear88.forward(layernormalization52_out1);
        let gelu30_out1 = burn::tensor::activation::gelu(linear88_out1);
        let linear89_out1 = self.linear89.forward(gelu30_out1);
        let add136_out1 = add135_out1.add(linear89_out1);
        let layernormalization53_out1 = {
            let dtype = add136_out1.clone().dtype();
            self.layernormalization53
                .forward(add136_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear90_out1 = self.linear90.forward(layernormalization53_out1);
        let split_tensors = linear90_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split23_out1, split23_out2, split23_out3] = split_tensors.try_into().unwrap();
        let constant257_out1 = self.constant257.val();
        let add137_out1 = split23_out1.add((constant257_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape134_out1 = add137_out1.reshape(concat6_out1);
        let transpose128_out1 = reshape134_out1.permute([0, 2, 1, 3]);
        let constant255_out1 = self.constant255.val();
        let add138_out1 = split23_out2.add((constant255_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape135_out1 = add138_out1.reshape(concat6_out1);
        let transpose129_out1 = reshape135_out1.permute([0, 2, 1, 3]);
        let constant256_out1 = self.constant256.val();
        let add139_out1 = split23_out3.add((constant256_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape136_out1 = add139_out1.reshape(concat6_out1);
        let transpose130_out1 = reshape136_out1.permute([0, 2, 1, 3]);
        let shape25_out1: [i64; 4] = {
            let axes = &transpose129_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice25_out1: [i64; 1] = shape25_out1[2..3].try_into().unwrap();
        let concat29_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice25_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape137_out1 = transpose129_out1.reshape(concat29_out1);
        let transpose131_out1 = reshape137_out1.permute([0, 2, 1]);
        let reshape138_out1 = transpose131_out1.reshape([1, 16, 64, -1]);
        let mul45_out1 = transpose128_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul46_out1 = reshape138_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul135_out1 = mul45_out1.matmul(mul46_out1);
        let add140_out1 = matmul135_out1.add(where1_out1.clone());
        let softmax23_out1 = burn::tensor::activation::softmax(add140_out1, 3);
        let isnan23_out1 = softmax23_out1.clone().is_nan();
        let where24_out1 = softmax23_out1.mask_fill(isnan23_out1, constant364_out1);
        let matmul136_out1 = where24_out1.matmul(transpose130_out1);
        let transpose132_out1 = matmul136_out1.permute([0, 2, 1, 3]);
        let reshape139_out1 = transpose132_out1.reshape(concat8_out1);
        let linear91_out1 = self.linear91.forward(reshape139_out1);
        let add141_out1 = add136_out1.add(linear91_out1);
        let layernormalization54_out1 = {
            let dtype = add141_out1.clone().dtype();
            self.layernormalization54
                .forward(add141_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear92_out1 = self.linear92.forward(layernormalization54_out1);
        let gelu31_out1 = burn::tensor::activation::gelu(linear92_out1);
        let linear93_out1 = self.linear93.forward(gelu31_out1);
        let add142_out1 = add141_out1.add(linear93_out1);
        let layernormalization55_out1 = {
            let dtype = add142_out1.clone().dtype();
            self.layernormalization55
                .forward(add142_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear94_out1 = self.linear94.forward(layernormalization55_out1);
        let split_tensors = linear94_out1.split_with_sizes([1024, 1024, 1024].into(), 2);
        let [split24_out1, split24_out2, split24_out3] = split_tensors.try_into().unwrap();
        let constant267_out1 = self.constant267.val();
        let add143_out1 = split24_out1.add((constant267_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape140_out1 = add143_out1.reshape(concat6_out1);
        let transpose133_out1 = reshape140_out1.permute([0, 2, 1, 3]);
        let constant265_out1 = self.constant265.val();
        let add144_out1 = split24_out2.add((constant265_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape141_out1 = add144_out1.reshape(concat6_out1);
        let transpose134_out1 = reshape141_out1.permute([0, 2, 1, 3]);
        let constant266_out1 = self.constant266.val();
        let add145_out1 = split24_out3.add((constant266_out1).unsqueeze_dims(&[0isize, 1isize]));
        let reshape142_out1 = add145_out1.reshape(concat6_out1);
        let transpose135_out1 = reshape142_out1.permute([0, 2, 1, 3]);
        let shape26_out1: [i64; 4] = {
            let axes = &transpose134_out1.clone().dims()[0..4];
            let mut output = [0i64; 4];
            for i in 0..4 {
                output[i] = axes[i] as i64;
            }
            output
        };
        let slice26_out1: [i64; 1] = shape26_out1[2..3].try_into().unwrap();
        let concat30_out1: [i64; 3usize] = [
            &constant280_out1[..],
            &slice26_out1[..],
            &constant361_out1[..],
        ]
        .concat()
        .try_into()
        .unwrap();
        let reshape143_out1 = transpose134_out1.reshape(concat30_out1);
        let transpose136_out1 = reshape143_out1.permute([0, 2, 1]);
        let reshape144_out1 = transpose136_out1.reshape([1, 16, 64, -1]);
        let mul47_out1 = transpose133_out1
            .mul((constant282_out1.clone()).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let mul48_out1 =
            reshape144_out1.mul((constant282_out1).unsqueeze_dims(&[0isize, 1isize, 2isize]));
        let matmul141_out1 = mul47_out1.matmul(mul48_out1);
        let add146_out1 = matmul141_out1.add(where1_out1);
        let softmax24_out1 = burn::tensor::activation::softmax(add146_out1, 3);
        let isnan24_out1 = softmax24_out1.clone().is_nan();
        let where25_out1 = softmax24_out1.mask_fill(isnan24_out1, constant364_out1);
        let matmul142_out1 = where25_out1.matmul(transpose135_out1);
        let transpose137_out1 = matmul142_out1.permute([0, 2, 1, 3]);
        let reshape145_out1 = transpose137_out1.reshape(concat8_out1);
        let linear95_out1 = self.linear95.forward(reshape145_out1);
        let add147_out1 = add142_out1.add(linear95_out1);
        let layernormalization56_out1 = {
            let dtype = add147_out1.clone().dtype();
            self.layernormalization56
                .forward(add147_out1.clone().cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear96_out1 = self.linear96.forward(layernormalization56_out1);
        let gelu32_out1 = burn::tensor::activation::gelu(linear96_out1);
        let linear97_out1 = self.linear97.forward(gelu32_out1);
        let add148_out1 = add147_out1.add(linear97_out1);
        let layernormalization57_out1 = {
            let dtype = add148_out1.dtype();
            self.layernormalization57
                .forward(add148_out1.cast(burn::tensor::DType::F32))
                .cast(dtype)
        };
        let linear98_out1 = self.linear98.forward(layernormalization57_out1);
        linear98_out1
    }
}

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    submodule1: Submodule1<B>,
    submodule2: Submodule2<B>,
    submodule3: Submodule3<B>,
    submodule4: Submodule4<B>,
    submodule5: Submodule5<B>,
    submodule6: Submodule6<B>,
    submodule7: Submodule7<B>,
    submodule8: Submodule8<B>,
    submodule9: Submodule9<B>,
    submodule10: Submodule10<B>,
    submodule11: Submodule11<B>,
    submodule12: Submodule12<B>,
    submodule13: Submodule13<B>,
    phantom: core::marker::PhantomData<B>,
    #[module(skip)]
    device: B::Device,
}

extern crate std;

impl<B: Backend> Default for Model<B> {
    fn default() -> Self {
        Self::from_file(
            "/disks/wizard/dev/Python/Projekty/AI/Sepple/target/release/build/multipa-model-d12b9d020bfa389c/out/model/multipa_sim.bpk",
            &Default::default(),
        )
    }
}

impl<B: Backend> Model<B> {
    /// Load model weights from a burnpack file.
    pub fn from_file<P: AsRef<std::path::Path>>(file: P, device: &B::Device) -> Self {
        let mut model = Self::new(device);
        let mut store = BurnpackStore::from_file(file);
        model
            .load_from(&mut store)
            .expect("Failed to load burnpack file");
        model
    }

    /// Load model weights from in-memory bytes.
    ///
    /// The bytes must be the contents of a `.bpk` file.
    pub fn from_bytes(bytes: Bytes, device: &B::Device) -> Self {
        let mut model = Self::new(device);
        let mut store = BurnpackStore::from_bytes(Some(bytes));
        model
            .load_from(&mut store)
            .expect("Failed to load burnpack bytes");
        model
    }
}

impl<B: Backend> Model<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let submodule1 = Submodule1::new(device);
        let submodule2 = Submodule2::new(device);
        let submodule3 = Submodule3::new(device);
        let submodule4 = Submodule4::new(device);
        let submodule5 = Submodule5::new(device);
        let submodule6 = Submodule6::new(device);
        let submodule7 = Submodule7::new(device);
        let submodule8 = Submodule8::new(device);
        let submodule9 = Submodule9::new(device);
        let submodule10 = Submodule10::new(device);
        let submodule11 = Submodule11::new(device);
        let submodule12 = Submodule12::new(device);
        let submodule13 = Submodule13::new(device);
        Self {
            submodule1,
            submodule2,
            submodule3,
            submodule4,
            submodule5,
            submodule6,
            submodule7,
            submodule8,
            submodule9,
            submodule10,
            submodule11,
            submodule12,
            submodule13,
            phantom: core::marker::PhantomData,
            device: device.clone(),
        }
    }

    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, input_values: Tensor<B, 2>) -> Tensor<B, 3> {
        let (add4_out1, shape1_out1, add3_out1, expand1_out1, constant360_out1) =
            self.submodule1.forward(input_values);
        let (
            add15_out1,
            concat6_out1,
            constant280_out1,
            constant361_out1,
            constant282_out1,
            where1_out1,
            constant364_out1,
            concat8_out1,
        ) = self.submodule2.forward(
            add4_out1,
            shape1_out1,
            add3_out1,
            expand1_out1,
            constant360_out1,
        );
        let add27_out1 = self.submodule3.forward(
            add15_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add39_out1 = self.submodule4.forward(
            add27_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add51_out1 = self.submodule5.forward(
            add39_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add63_out1 = self.submodule6.forward(
            add51_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add75_out1 = self.submodule7.forward(
            add63_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add87_out1 = self.submodule8.forward(
            add75_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add99_out1 = self.submodule9.forward(
            add87_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add111_out1 = self.submodule10.forward(
            add99_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add123_out1 = self.submodule11.forward(
            add111_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let add135_out1 = self.submodule12.forward(
            add123_out1,
            concat6_out1.clone(),
            constant280_out1.clone(),
            constant361_out1.clone(),
            constant282_out1.clone(),
            where1_out1.clone(),
            constant364_out1.clone(),
            concat8_out1.clone(),
        );
        let linear98_out1 = self.submodule13.forward(
            add135_out1,
            concat6_out1,
            constant280_out1,
            constant361_out1,
            constant282_out1,
            where1_out1,
            constant364_out1,
            concat8_out1,
        );
        linear98_out1
    }
}
