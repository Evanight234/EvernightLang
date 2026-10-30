use crate::matra::larik::MatraArray;

use std::cell::RefCell;

use std::fs::File;

use std::io::{BufRead, BufReader, Write};

use std::rc::Rc;



/// Generator angka acak LCG deterministik untuk inisialisasi bobot dan dropout

struct SimpleRng {

    state: u64,

}



impl SimpleRng {

    fn new(seed: u64) -> Self {

        Self {

            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },

        }

    }



    fn next_u64(&mut self) -> u64 {

        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);

        self.state

    }



    fn next_f64(&mut self) -> f64 {

        let val = (self.next_u64() >> 11) as f64;

        val / ((1u64 << 53) as f64)

    }



    fn uniform(&mut self, min: f64, max: f64) -> f64 {

        min + (max - min) * self.next_f64()

    }

}



/// Struktur Parameter pembawa bobot atau bias dengan pelacakan gradien

#[derive(Debug, Clone, PartialEq)]

pub struct Parameter {

    pub nama: String,

    pub data: Vec<f64>,

    pub bentuk: Vec<usize>,

    pub grad: Vec<f64>,

}



impl Parameter {

    pub fn new(nama: String, data: Vec<f64>, bentuk: Vec<usize>) -> Self {

        let len = data.len();

        Self {

            nama,

            data,

            bentuk,

            grad: vec![0.0; len],

        }

    }



    pub fn nol_gradien(&mut self) {

        for g in self.grad.iter_mut() {

            *g = 0.0;

        }

    }

}



/// Jenis-jenis lapisan jaringan syaraf tiruan

#[derive(Debug, Clone, PartialEq)]

pub enum JenisLapisan {

    Linear {

        in_fitur: usize,

        out_fitur: usize,

        bobot: Parameter,

        bias: Parameter,

        input_terakhir: Option<MatraArray>,

    },

    Relu {

        mask_positif: Option<Vec<bool>>,

    },

    Sigmoid {

        output_terakhir: Option<MatraArray>,

    },

    Tanh {

        output_terakhir: Option<MatraArray>,

    },

    Softmax {

        output_terakhir: Option<MatraArray>,

    },

    Dropout {

        probabilitas: f64,

        mask: Option<Vec<f64>>,

        training: bool,

    },

}



/// Struktur pembungkus lapisan jaringan syaraf tiruan

#[derive(Debug, Clone, PartialEq)]

pub struct MatraLapisan {

    pub jenis: JenisLapisan,

}



impl MatraLapisan {

    /// Membuat lapisan Linear dengan inisialisasi bobot He / Xavier uniform

    pub fn linear(in_fitur: usize, out_fitur: usize, seed: Option<u64>) -> Self {

        let mut rng = SimpleRng::new(seed.unwrap_or(42));

        let batas = (6.0 / (in_fitur + out_fitur) as f64).sqrt();

        let total = in_fitur * out_fitur;

        let mut bobot_data = Vec::with_capacity(total);

        for _ in 0..total {

            bobot_data.push(rng.uniform(-batas, batas));

        }

        let bias_data = vec![0.0; out_fitur];



        let bobot = Parameter::new("bobot".to_string(), bobot_data, vec![in_fitur, out_fitur]);

        let bias = Parameter::new("bias".to_string(), bias_data, vec![1, out_fitur]);



        Self {

            jenis: JenisLapisan::Linear {

                in_fitur,

                out_fitur,

                bobot,

                bias,

                input_terakhir: None,

            },

        }

    }



    pub fn relu() -> Self {

        Self {

            jenis: JenisLapisan::Relu { mask_positif: None },

        }

    }



    pub fn sigmoid() -> Self {

        Self {

            jenis: JenisLapisan::Sigmoid { output_terakhir: None },

        }

    }



    pub fn tanh() -> Self {

        Self {

            jenis: JenisLapisan::Tanh { output_terakhir: None },

        }

    }



    pub fn softmax() -> Self {

        Self {

            jenis: JenisLapisan::Softmax { output_terakhir: None },

        }

    }



    pub fn dropout(probabilitas: f64) -> Self {

        Self {

            jenis: JenisLapisan::Dropout {

                probabilitas,

                mask: None,

                training: true,

            },

        }

    }



    /// Forward pass untuk satu lapisan

    pub fn maju(&mut self, input: &MatraArray, training: bool) -> Result<MatraArray, String> {

        match &mut self.jenis {

            JenisLapisan::Linear { in_fitur, out_fitur, bobot, bias, input_terakhir } => {

                let (baris, kolom) = match input.bentuk.as_slice() {

                    [b, k] => (*b, *k),

                    [k] => (1, *k),

                    _ => return Err("Input untuk Linear harus 1D atau 2D".to_string()),

                };



                if kolom != *in_fitur {

                    return Err(format!(

                        "Ketidakcocokan dimensi Linear: input memiliki {} fitur, diharapkan {}",

                        kolom, in_fitur

                    ));

                }



                *input_terakhir = Some(input.clone());



                let mut out_data = vec![0.0; baris * *out_fitur];

                for i in 0..baris {

                    for j in 0..*out_fitur {

                        let mut sum = bias.data[j];

                        for k in 0..*in_fitur {

                            sum += input.data[i * kolom + k] * bobot.data[k * *out_fitur + j];

                        }

                        out_data[i * *out_fitur + j] = sum;

                    }

                }



                Ok(MatraArray::dari_vektor_2d(out_data, baris, *out_fitur))

            }



            JenisLapisan::Relu { mask_positif } => {

                let mut out_data = Vec::with_capacity(input.data.len());

                let mut mask = Vec::with_capacity(input.data.len());

                for &v in &input.data {

                    if v > 0.0 {

                        out_data.push(v);

                        mask.push(true);

                    } else {

                        out_data.push(0.0);

                        mask.push(false);

                    }

                }

                *mask_positif = Some(mask);

                Ok(MatraArray::dari_vektor(out_data, input.bentuk.clone()))

            }



            JenisLapisan::Sigmoid { output_terakhir } => {

                let mut out_data = Vec::with_capacity(input.data.len());

                for &v in &input.data {

                    let sig = 1.0 / (1.0 + (-v).exp());

                    out_data.push(sig);

                }

                let res = MatraArray::dari_vektor(out_data, input.bentuk.clone());

                *output_terakhir = Some(res.clone());

                Ok(res)

            }



            JenisLapisan::Tanh { output_terakhir } => {

                let mut out_data = Vec::with_capacity(input.data.len());

                for &v in &input.data {

                    out_data.push(v.tanh());

                }

                let res = MatraArray::dari_vektor(out_data, input.bentuk.clone());

                *output_terakhir = Some(res.clone());

                Ok(res)

            }



            JenisLapisan::Softmax { output_terakhir } => {

                let (baris, kolom) = match input.bentuk.as_slice() {

                    [b, k] => (*b, *k),

                    [k] => (1, *k),

                    _ => return Err("Input untuk Softmax harus 1D atau 2D".to_string()),

                };



                let mut out_data = vec![0.0; baris * kolom];

                for i in 0..baris {

                    let mut max_val = f64::NEG_INFINITY;

                    for j in 0..kolom {

                        let v = input.data[i * kolom + j];

                        if v > max_val {

                            max_val = v;

                        }

                    }



                    let mut sum_exp = 0.0;

                    for j in 0..kolom {

                        let exp_val = (input.data[i * kolom + j] - max_val).exp();

                        out_data[i * kolom + j] = exp_val;

                        sum_exp += exp_val;

                    }



                    for j in 0..kolom {

                        out_data[i * kolom + j] /= sum_exp;

                    }

                }



                let res = MatraArray::dari_vektor_2d(out_data, baris, kolom);

                *output_terakhir = Some(res.clone());

                Ok(res)

            }



            JenisLapisan::Dropout { probabilitas, mask, training: mode_latih } => {

                *mode_latih = training;

                if !training || *probabilitas <= 0.0 {

                    *mask = None;

                    return Ok(input.clone());

                }



                let p = *probabilitas;

                let skala = 1.0 / (1.0 - p);

                let mut rng = SimpleRng::new(1337);

                let mut mask_vec = Vec::with_capacity(input.data.len());

                let mut out_data = Vec::with_capacity(input.data.len());



                for &v in &input.data {

                    if rng.next_f64() >= p {

                        mask_vec.push(skala);

                        out_data.push(v * skala);

                    } else {

                        mask_vec.push(0.0);

                        out_data.push(0.0);

                    }

                }



                *mask = Some(mask_vec);

                Ok(MatraArray::dari_vektor(out_data, input.bentuk.clone()))

            }

        }

    }



    /// Backward pass untuk satu lapisan: menghitung gradien ke input dan mengakumulasikan gradien ke bobot/bias

    pub fn mundur(&mut self, grad_output: &MatraArray) -> Result<MatraArray, String> {

        match &mut self.jenis {

            JenisLapisan::Linear { in_fitur, out_fitur, bobot, bias, input_terakhir } => {

                let input = match input_terakhir {

                    Some(inp) => inp,

                    None => return Err("Forward pass harus dijalankan sebelum mundur() pada Linear".to_string()),

                };



                let (baris, _) = match input.bentuk.as_slice() {

                    [b, k] => (*b, *k),

                    [k] => (1, *k),

                    _ => (1, input.data.len()),

                };



                // 1. Akumulasi gradien ke bobot: grad_W = X^T * grad_output

                for k in 0..*in_fitur {

                    for j in 0..*out_fitur {

                        let mut sum = 0.0;

                        for i in 0..baris {

                            sum += input.data[i * *in_fitur + k] * grad_output.data[i * *out_fitur + j];

                        }

                        bobot.grad[k * *out_fitur + j] += sum;

                    }

                }



                // 2. Akumulasi gradien ke bias: grad_b = sum(grad_output, axis=0)

                for j in 0..*out_fitur {

                    let mut sum = 0.0;

                    for i in 0..baris {

                        sum += grad_output.data[i * *out_fitur + j];

                    }

                    bias.grad[j] += sum;

                }



                // 3. Hitung gradien ke input: grad_X = grad_output * W^T

                let mut grad_input = vec![0.0; baris * *in_fitur];

                for i in 0..baris {

                    for k in 0..*in_fitur {

                        let mut sum = 0.0;

                        for j in 0..*out_fitur {

                            sum += grad_output.data[i * *out_fitur + j] * bobot.data[k * *out_fitur + j];

                        }

                        grad_input[i * *in_fitur + k] = sum;

                    }

                }



                Ok(MatraArray::dari_vektor_2d(grad_input, baris, *in_fitur))

            }



            JenisLapisan::Relu { mask_positif } => {

                let mask = match mask_positif {

                    Some(m) => m,

                    None => return Err("Forward pass harus dijalankan sebelum mundur() pada Relu".to_string()),

                };



                let mut grad_input = Vec::with_capacity(grad_output.data.len());

                for (i, &g) in grad_output.data.iter().enumerate() {

                    if mask[i] {

                        grad_input.push(g);

                    } else {

                        grad_input.push(0.0);

                    }

                }



                Ok(MatraArray::dari_vektor(grad_input, grad_output.bentuk.clone()))

            }



            JenisLapisan::Sigmoid { output_terakhir } => {

                let out = match output_terakhir {

                    Some(o) => o,

                    None => return Err("Forward pass harus dijalankan sebelum mundur() pada Sigmoid".to_string()),

                };



                let mut grad_input = Vec::with_capacity(grad_output.data.len());

                for (i, &g) in grad_output.data.iter().enumerate() {

                    let s = out.data[i];

                    grad_input.push(g * s * (1.0 - s));

                }



                Ok(MatraArray::dari_vektor(grad_input, grad_output.bentuk.clone()))

            }



            JenisLapisan::Tanh { output_terakhir } => {

                let out = match output_terakhir {

                    Some(o) => o,

                    None => return Err("Forward pass harus dijalankan sebelum mundur() pada Tanh".to_string()),

                };



                let mut grad_input = Vec::with_capacity(grad_output.data.len());

                for (i, &g) in grad_output.data.iter().enumerate() {

                    let t = out.data[i];

                    grad_input.push(g * (1.0 - t * t));

                }



                Ok(MatraArray::dari_vektor(grad_input, grad_output.bentuk.clone()))

            }



            JenisLapisan::Softmax { output_terakhir } => {

                let out = match output_terakhir {

                    Some(o) => o,

                    None => return Err("Forward pass harus dijalankan sebelum mundur() pada Softmax".to_string()),

                };



                let (baris, kolom) = match out.bentuk.as_slice() {

                    [b, k] => (*b, *k),

                    [k] => (1, *k),

                    _ => (1, out.data.len()),

                };



                let mut grad_input = vec![0.0; baris * kolom];

                for i in 0..baris {

                    let mut sum_dot = 0.0;

                    for j in 0..kolom {

                        sum_dot += grad_output.data[i * kolom + j] * out.data[i * kolom + j];

                    }



                    for j in 0..kolom {

                        let s = out.data[i * kolom + j];

                        let g = grad_output.data[i * kolom + j];

                        grad_input[i * kolom + j] = s * (g - sum_dot);

                    }

                }



                Ok(MatraArray::dari_vektor_2d(grad_input, baris, kolom))

            }



            JenisLapisan::Dropout { mask, training, .. } => {

                if !*training {

                    return Ok(grad_output.clone());

                }



                let m = match mask {

                    Some(mask_vec) => mask_vec,

                    None => return Ok(grad_output.clone()),

                };



                let mut grad_input = Vec::with_capacity(grad_output.data.len());

                for (i, &g) in grad_output.data.iter().enumerate() {

                    grad_input.push(g * m[i]);

                }



                Ok(MatraArray::dari_vektor(grad_input, grad_output.bentuk.clone()))

            }

        }

    }



    /// Mengembalikan referensi mutable ke parameter lapisan

    pub fn parameter_mut(&mut self) -> Vec<&mut Parameter> {

        match &mut self.jenis {

            JenisLapisan::Linear { bobot, bias, .. } => vec![bobot, bias],

            _ => vec![],

        }

    }

}



/// Struktur Jaringan Syaraf Tiruan Sekuensial (Multi-Layer Perceptron)

#[derive(Debug, Clone, PartialEq)]

pub struct MatraJaringan {

    pub lapisan: Vec<MatraLapisan>,

    pub training: bool,

}



impl MatraJaringan {

    pub fn new(lapisan: Vec<MatraLapisan>) -> Self {

        Self {

            lapisan,

            training: true,

        }

    }



    pub fn mode_latih(&mut self) {

        self.training = true;

    }



    pub fn mode_evaluasi(&mut self) {

        self.training = false;

    }



    /// Menjalankan forward pass berurutan melalui semua lapisan

    pub fn maju(&mut self, input: &MatraArray) -> Result<MatraArray, String> {

        let mut curr = input.clone();

        for lap in self.lapisan.iter_mut() {

            curr = lap.maju(&curr, self.training)?;

        }

        Ok(curr)

    }



    /// Menjalankan backward pass berurutan terbalik

    pub fn mundur(&mut self, grad_output: &MatraArray) -> Result<MatraArray, String> {

        let mut curr = grad_output.clone();

        for lap in self.lapisan.iter_mut().rev() {

            curr = lap.mundur(&curr)?;

        }

        Ok(curr)

    }



    /// Melakukan prediksi dalam mode evaluasi

    pub fn prediksi(&mut self, input: &MatraArray) -> Result<MatraArray, String> {

        let prev_mode = self.training;

        self.mode_evaluasi();

        let res = self.maju(input);

        self.training = prev_mode;

        res

    }



    /// Mengumpulkan seluruh parameter mutable

    pub fn parameter_mut(&mut self) -> Vec<&mut Parameter> {

        let mut params = Vec::new();

        for lap in self.lapisan.iter_mut() {

            for p in lap.parameter_mut() {

                params.push(p);

            }

        }

        params

    }



    /// Mengosongkan akumulasi gradien pada semua parameter

    pub fn nol_gradien(&mut self) {

        for p in self.parameter_mut() {

            p.nol_gradien();

        }

    }



    /// Menyimpan bobot parameter model ke format berkas `.matra`

    pub fn simpan<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), String> {

        let mut f = File::create(&path).map_err(|e| format!("Gagal menyimpan model ke '{}': {}", path.as_ref().display(), e))?;

        writeln!(f, "EVERNIGHT_MATRA_MODEL_V1").map_err(|e| e.to_string())?;



        let params = self.parameter_mut();

        writeln!(f, "JUMLAH_PARAM:{}", params.len()).map_err(|e| e.to_string())?;



        for p in params {

            let bentuk_str = p.bentuk.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(",");

            writeln!(f, "PARAM:{}:{}:{}", p.nama, bentuk_str, p.data.len()).map_err(|e| e.to_string())?;

            let data_str = p.data.iter().map(|d| format!("{:.16}", d)).collect::<Vec<_>>().join(",");

            writeln!(f, "{}", data_str).map_err(|e| e.to_string())?;

        }

        Ok(())

    }



    /// Memuat bobot parameter dari berkas `.matra`

    pub fn muat<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), String> {

        let f = File::open(&path).map_err(|e| format!("Gagal membuka berkas model '{}': {}", path.as_ref().display(), e))?;

        let reader = BufReader::new(f);

        let mut lines = reader.lines();



        let header = lines.next().ok_or("Berkas model kosong")?.map_err(|e| e.to_string())?;

        if header.trim() != "EVERNIGHT_MATRA_MODEL_V1" {

            return Err("Format berkas model bukan EVERNIGHT_MATRA_MODEL_V1".to_string());

        }



        let _jumlah_line = lines.next().ok_or("Format tidak lengkap")?.map_err(|e| e.to_string())?;



        let mut params = self.parameter_mut();

        let mut idx = 0;



        while let Some(meta_res) = lines.next() {

            let meta = meta_res.map_err(|e| e.to_string())?;

            if !meta.starts_with("PARAM:") {

                continue;

            }



            let data_line = lines.next().ok_or("Data parameter hilang")?.map_err(|e| e.to_string())?;

            let values: Result<Vec<f64>, _> = data_line

                .split(',')

                .map(|s| s.trim().parse::<f64>())

                .collect();

            let parsed_values = values.map_err(|e| format!("Kesalahan parsing float: {}", e))?;



            if idx < params.len() {

                if params[idx].data.len() == parsed_values.len() {

                    params[idx].data = parsed_values;

                } else {

                    return Err(format!(

                        "Ukuran parameter ke-{} tidak cocok (berkas: {}, model: {})",

                        idx, parsed_values.len(), params[idx].data.len()

                    ));

                }

                idx += 1;

            }

        }



        Ok(())

    }

}



/// Fungsi Kerugian (Loss Functions)

#[derive(Debug, Clone, PartialEq)]

pub enum JenisLoss {

    MSE,

    BCE,

    CrossEntropy,

}



#[derive(Debug, Clone, PartialEq)]

pub struct MatraLoss {

    pub jenis: JenisLoss,

}



impl MatraLoss {

    pub fn mse() -> Self {

        Self { jenis: JenisLoss::MSE }

    }



    pub fn bce() -> Self {

        Self { jenis: JenisLoss::BCE }

    }



    pub fn cross_entropy() -> Self {

        Self { jenis: JenisLoss::CrossEntropy }

    }



    /// Menghitung nilai scalar loss

    pub fn hitung(&self, pred: &MatraArray, target: &MatraArray) -> Result<f64, String> {

        if pred.data.len() != target.data.len() {

            return Err(format!(

                "Jumlah elemen prediksi ({}) dan target ({}) harus sama",

                pred.data.len(), target.data.len()

            ));

        }



        let n = pred.data.len() as f64;

        let eps = 1e-15;



        match self.jenis {

            JenisLoss::MSE => {

                let mut sum = 0.0;

                for i in 0..pred.data.len() {

                    let diff = pred.data[i] - target.data[i];

                    sum += diff * diff;

                }

                Ok(sum / n)

            }



            JenisLoss::BCE => {

                let mut sum = 0.0;

                for i in 0..pred.data.len() {

                    let p = pred.data[i].clamp(eps, 1.0 - eps);

                    let y = target.data[i];

                    sum += -(y * p.ln() + (1.0 - y) * (1.0 - p).ln());

                }

                Ok(sum / n)

            }



            JenisLoss::CrossEntropy => {

                let (baris, kolom) = match pred.bentuk.as_slice() {

                    [b, k] => (*b, *k),

                    [k] => (1, *k),

                    _ => (1, pred.data.len()),

                };



                let mut sum = 0.0;

                for i in 0..baris {

                    for j in 0..kolom {

                        let y = target.data[i * kolom + j];

                        if y > 0.0 {

                            let p = pred.data[i * kolom + j].clamp(eps, 1.0);

                            sum += -y * p.ln();

                        }

                    }

                }

                Ok(sum / (baris as f64))

            }

        }

    }



    /// Menghitung gradien loss terhadap prediksi: dL/d(pred)

    pub fn gradien(&self, pred: &MatraArray, target: &MatraArray) -> Result<MatraArray, String> {

        if pred.data.len() != target.data.len() {

            return Err("Ukuran prediksi dan target harus sama untuk gradien loss".to_string());

        }



        let n = pred.data.len() as f64;

        let eps = 1e-15;



        match self.jenis {

            JenisLoss::MSE => {

                let mut grad = Vec::with_capacity(pred.data.len());

                for i in 0..pred.data.len() {

                    grad.push(2.0 * (pred.data[i] - target.data[i]) / n);

                }

                Ok(MatraArray::dari_vektor(grad, pred.bentuk.clone()))

            }



            JenisLoss::BCE => {

                let mut grad = Vec::with_capacity(pred.data.len());

                for i in 0..pred.data.len() {

                    let p = pred.data[i].clamp(eps, 1.0 - eps);

                    let y = target.data[i];

                    let g = (p - y) / (p * (1.0 - p) * n);

                    grad.push(g);

                }

                Ok(MatraArray::dari_vektor(grad, pred.bentuk.clone()))

            }



            JenisLoss::CrossEntropy => {

                let (baris, kolom) = match pred.bentuk.as_slice() {

                    [b, k] => (*b, *k),

                    [k] => (1, *k),

                    _ => (1, pred.data.len()),

                };



                let b_f64 = baris as f64;

                let mut grad = Vec::with_capacity(pred.data.len());

                for i in 0..baris {

                    for j in 0..kolom {

                        let y = target.data[i * kolom + j];

                        let p = pred.data[i * kolom + j].clamp(eps, 1.0);

                        grad.push((-y / p) / b_f64);

                    }

                }

                Ok(MatraArray::dari_vektor_2d(grad, baris, kolom))

            }

        }

    }

}



/// Pengoptimal Jaringan Syaraf

#[derive(Debug, Clone, PartialEq)]

pub enum JenisOptimizer {

    SGD {

        lr: f64,

        momentum: f64,

        velocities: Vec<Vec<f64>>,

    },

    Adam {

        lr: f64,

        beta1: f64,

        beta2: f64,

        eps: f64,

        m: Vec<Vec<f64>>,

        v: Vec<Vec<f64>>,

        t: usize,

    },

}



#[derive(Debug, Clone, PartialEq)]

pub struct MatraOptimizer {

    pub model: Rc<RefCell<MatraJaringan>>,

    pub jenis: JenisOptimizer,

}



impl MatraOptimizer {

    pub fn sgd(model: Rc<RefCell<MatraJaringan>>, lr: f64, momentum: Option<f64>) -> Self {

        let m_val = momentum.unwrap_or(0.0);

        let n_params = model.borrow_mut().parameter_mut().len();

        let velocities = vec![Vec::new(); n_params];



        Self {

            model,

            jenis: JenisOptimizer::SGD {

                lr,

                momentum: m_val,

                velocities,

            },

        }

    }



    pub fn adam(model: Rc<RefCell<MatraJaringan>>, lr: f64, beta1: Option<f64>, beta2: Option<f64>) -> Self {

        let b1 = beta1.unwrap_or(0.9);

        let b2 = beta2.unwrap_or(0.999);

        let n_params = model.borrow_mut().parameter_mut().len();

        let m = vec![Vec::new(); n_params];

        let v = vec![Vec::new(); n_params];



        Self {

            model,

            jenis: JenisOptimizer::Adam {

                lr,

                beta1: b1,

                beta2: b2,

                eps: 1e-8,

                m,

                v,

                t: 0,

            },

        }

    }



    /// Mengosongkan akumulasi gradien

    pub fn nol_gradien(&mut self) -> Result<(), String> {

        self.model.borrow_mut().nol_gradien();

        Ok(())

    }



    /// Melakukan update parameter (step)

    pub fn langkah(&mut self) -> Result<(), String> {

        let mut model_ref = self.model.borrow_mut();

        let mut params = model_ref.parameter_mut();



        match &mut self.jenis {

            JenisOptimizer::SGD { lr, momentum, velocities } => {

                if velocities.len() != params.len() {

                    velocities.resize(params.len(), Vec::new());

                }



                for (idx, p) in params.iter_mut().enumerate() {

                    if velocities[idx].len() != p.data.len() {

                        velocities[idx] = vec![0.0; p.data.len()];

                    }



                    for i in 0..p.data.len() {

                        let g = p.grad[i];

                        velocities[idx][i] = *momentum * velocities[idx][i] + *lr * g;

                        p.data[i] -= velocities[idx][i];

                    }

                }

            }



            JenisOptimizer::Adam { lr, beta1, beta2, eps, m, v, t } => {

                *t += 1;

                let step = *t as f64;

                let b1_pow = beta1.powf(step);

                let b2_pow = beta2.powf(step);



                if m.len() != params.len() {

                    m.resize(params.len(), Vec::new());

                    v.resize(params.len(), Vec::new());

                }



                for (idx, p) in params.iter_mut().enumerate() {

                    if m[idx].len() != p.data.len() {

                        m[idx] = vec![0.0; p.data.len()];

                        v[idx] = vec![0.0; p.data.len()];

                    }



                    for i in 0..p.data.len() {

                        let g = p.grad[i];

                        m[idx][i] = *beta1 * m[idx][i] + (1.0 - *beta1) * g;

                        v[idx][i] = *beta2 * v[idx][i] + (1.0 - *beta2) * g * g;



                        let m_hat = m[idx][i] / (1.0 - b1_pow);

                        let v_hat = v[idx][i] / (1.0 - b2_pow);



                        p.data[i] -= *lr * m_hat / (v_hat.sqrt() + *eps);

                    }

                }

            }

        }



        Ok(())

    }

}

