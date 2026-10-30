use evernight_vm::matra::larik::MatraArray;
use evernight_vm::matra::pelatih::{MatraCallback, MatraPelatih};
use evernight_vm::matra::syaraf::{MatraJaringan, MatraLapisan, MatraLoss, MatraOptimizer};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_linear_forward_dan_backward() {
    let mut linear = MatraLapisan::linear(2, 2, Some(42));
    let x = MatraArray::dari_vektor_2d(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

    let out = linear.maju(&x, true).unwrap();
    assert_eq!(out.bentuk, vec![2, 2]);

    let grad_out = MatraArray::dari_vektor_2d(vec![0.5, -0.5, 1.0, -1.0], 2, 2);
    let grad_in = linear.mundur(&grad_out).unwrap();
    assert_eq!(grad_in.bentuk, vec![2, 2]);

    let params = linear.parameter_mut();
    assert_eq!(params.len(), 2); // bobot & bias
    assert!(params[0].grad.iter().any(|&g| g != 0.0));
    assert!(params[1].grad.iter().any(|&g| g != 0.0));
}

#[test]
fn test_aktivasi_dan_derivatif() {
    // 1. Relu
    let mut relu = MatraLapisan::relu();
    let x = MatraArray::dari_vektor_2d(vec![-2.0, 0.0, 3.0, 5.0], 2, 2);
    let out = relu.maju(&x, true).unwrap();
    assert_eq!(out.data, vec![0.0, 0.0, 3.0, 5.0]);

    let grad_out = MatraArray::dari_vektor_2d(vec![1.0, 1.0, 1.0, 1.0], 2, 2);
    let grad_in = relu.mundur(&grad_out).unwrap();
    assert_eq!(grad_in.data, vec![0.0, 0.0, 1.0, 1.0]);

    // 2. Sigmoid
    let mut sig = MatraLapisan::sigmoid();
    let x_zero = MatraArray::dari_vektor_2d(vec![0.0], 1, 1);
    let out_sig = sig.maju(&x_zero, false).unwrap();
    assert!((out_sig.data[0] - 0.5).abs() < 1e-6);

    let grad_sig_in = sig
        .mundur(&MatraArray::dari_vektor_2d(vec![1.0], 1, 1))
        .unwrap();
    assert!((grad_sig_in.data[0] - 0.25).abs() < 1e-6); // s*(1-s) = 0.5*0.5 = 0.25

    // 3. Tanh
    let mut tanh = MatraLapisan::tanh();
    let out_tanh = tanh.maju(&x_zero, false).unwrap();
    assert!((out_tanh.data[0] - 0.0).abs() < 1e-6);

    // 4. Softmax
    let mut sm = MatraLapisan::softmax();
    let x_sm = MatraArray::dari_vektor_2d(vec![1.0, 1.0], 1, 2);
    let out_sm = sm.maju(&x_sm, false).unwrap();
    assert!((out_sm.data[0] - 0.5).abs() < 1e-6);
    assert!((out_sm.data[1] - 0.5).abs() < 1e-6);
}

#[test]
fn test_dropout_eval_vs_train() {
    let mut drop = MatraLapisan::dropout(0.5);
    let x = MatraArray::dari_vektor_2d(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

    // Saat eval mode (training = false), dropout adalah identitas persis
    let out_eval = drop.maju(&x, false).unwrap();
    assert_eq!(out_eval.data, x.data);

    // Saat training mode, scaling dilakukan
    let out_train = drop.maju(&x, true).unwrap();
    assert_eq!(out_train.data.len(), 4);
}

#[test]
fn test_loss_mse_dan_bce() {
    let pred = MatraArray::dari_vektor_2d(vec![0.8, 0.2], 2, 1);
    let target = MatraArray::dari_vektor_2d(vec![1.0, 0.0], 2, 1);

    // MSE: ((0.8-1)^2 + (0.2-0)^2) / 2 = (0.04 + 0.04) / 2 = 0.04
    let loss_mse = MatraLoss::mse();
    let l_mse = loss_mse.hitung(&pred, &target).unwrap();
    assert!((l_mse - 0.04).abs() < 1e-6);

    let g_mse = loss_mse.gradien(&pred, &target).unwrap();
    assert_eq!(g_mse.data.len(), 2);

    // BCE
    let loss_bce = MatraLoss::bce();
    let l_bce = loss_bce.hitung(&pred, &target).unwrap();
    assert!(l_bce > 0.0);

    let g_bce = loss_bce.gradien(&pred, &target).unwrap();
    assert_eq!(g_bce.data.len(), 2);
}

#[test]
fn test_pengoptimal_adam_dan_sgd() {
    let l1 = MatraLapisan::linear(2, 1, Some(10));
    let model = Rc::new(RefCell::new(MatraJaringan::new(vec![l1])));
    let mut optim = MatraOptimizer::adam(model.clone(), 0.01, None, None);

    let x = MatraArray::dari_vektor_2d(vec![1.0, 1.0], 1, 2);
    let y = MatraArray::dari_vektor_2d(vec![5.0], 1, 1);
    let loss_fn = MatraLoss::mse();

    let initial_loss = {
        let pred = model.borrow_mut().maju(&x).unwrap();
        loss_fn.hitung(&pred, &y).unwrap()
    };

    // Jalankan beberapa langkah optimasi Adam
    for _ in 0..10 {
        optim.nol_gradien().unwrap();
        let pred = model.borrow_mut().maju(&x).unwrap();
        let grad = loss_fn.gradien(&pred, &y).unwrap();
        model.borrow_mut().mundur(&grad).unwrap();
        optim.langkah().unwrap();
    }

    let final_loss = {
        let pred = model.borrow_mut().maju(&x).unwrap();
        loss_fn.hitung(&pred, &y).unwrap()
    };

    assert!(
        final_loss < initial_loss,
        "Loss harus menurun setelah optimasi Adam (awal: {}, akhir: {})",
        initial_loss,
        final_loss
    );
}

#[test]
fn test_pelatih_xor_mlp_training() {
    // Problem Non-Linear XOR
    // [0, 0] -> 0
    // [0, 1] -> 1
    // [1, 0] -> 1
    // [1, 1] -> 0
    let x = MatraArray::dari_vektor_2d(vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0], 4, 2);

    let y = MatraArray::dari_vektor_2d(vec![0.0, 1.0, 1.0, 0.0], 4, 1);

    // Arsitektur Multi-Layer Perceptron: 2 -> 8 (Tanh) -> 1 (Sigmoid)
    let model = Rc::new(RefCell::new(MatraJaringan::new(vec![
        MatraLapisan::linear(2, 8, Some(123)),
        MatraLapisan::tanh(),
        MatraLapisan::linear(8, 1, Some(456)),
        MatraLapisan::sigmoid(),
    ])));

    let optim = Rc::new(RefCell::new(MatraOptimizer::adam(
        model.clone(),
        0.08,
        None,
        None,
    )));
    let loss = MatraLoss::bce();

    let mut pelatih = MatraPelatih::new(
        model.clone(),
        optim,
        loss,
        Some(vec!["akurasi".to_string()]),
    );

    // Latih selama 150 putaran
    let riwayat = pelatih.latih(&x, &y, 150, 4, None, None).unwrap();
    assert!(riwayat.contains_key("rugi"));

    let eval_res = pelatih.evaluasi(&x, &y).unwrap();
    let akurasi = *eval_res.get("akurasi").unwrap();
    let rugi_akhir = *eval_res.get("rugi").unwrap();

    assert!(
        rugi_akhir < 0.2,
        "Rugi akhir harus rendah (didapat {})",
        rugi_akhir
    );
    assert_eq!(
        akurasi, 1.0,
        "Model MLP harus menyelesaikan XOR dengan akurasi 100%"
    );
}

#[test]
fn test_callback_berhenti_awal_dan_simpan_model() {
    let x = MatraArray::dari_vektor_2d(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    let y = MatraArray::dari_vektor_2d(vec![1.0, 0.0], 2, 1);

    let model = Rc::new(RefCell::new(MatraJaringan::new(vec![
        MatraLapisan::linear(2, 4, Some(42)),
        MatraLapisan::relu(),
        MatraLapisan::linear(4, 1, Some(84)),
        MatraLapisan::sigmoid(),
    ])));

    let optim = Rc::new(RefCell::new(MatraOptimizer::adam(
        model.clone(),
        0.01,
        None,
        None,
    )));
    let mut pelatih = MatraPelatih::new(model.clone(), optim, MatraLoss::bce(), None);

    let model_path = "model_terbaik_test.matra";
    pelatih.pasang_callback(Rc::new(RefCell::new(MatraCallback::simpan_terbaik(
        model_path,
        Some("akurasi"),
        Some("maks"),
    ))));
    pelatih.pasang_callback(Rc::new(RefCell::new(MatraCallback::berhenti_awal(
        Some("rugi"),
        Some(3),
        Some(0.0001),
        Some("min"),
    ))));

    let riwayat = pelatih.latih(&x, &y, 50, 2, None, None).unwrap();
    let putaran_selesai = riwayat.get("putaran").unwrap().len();

    // Pastikan berkas model tersimpan dan dapat dimuat kembali
    assert!(
        std::path::Path::new(model_path).exists(),
        "Berkas model checkpoint harus terbentuk"
    );

    let mut model_baru = MatraJaringan::new(vec![
        MatraLapisan::linear(2, 4, None),
        MatraLapisan::relu(),
        MatraLapisan::linear(4, 1, None),
        MatraLapisan::sigmoid(),
    ]);

    model_baru.muat(model_path).unwrap();

    // Simpan model terkini dan muat ke model baru
    model.borrow_mut().simpan(model_path).unwrap();
    model_baru.muat(model_path).unwrap();

    let pred_asli = model.borrow_mut().prediksi(&x).unwrap();
    let pred_muat = model_baru.prediksi(&x).unwrap();

    for (p1, p2) in pred_asli.data.iter().zip(pred_muat.data.iter()) {
        assert!(
            (p1 - p2).abs() < 1e-5,
            "Prediksi model yang dimuat harus identik dengan model tersimpan (p1: {}, p2: {})",
            p1,
            p2
        );
    }

    // Bersihkan berkas uji
    let _ = std::fs::remove_file(model_path);
    assert!(putaran_selesai > 0);
}
