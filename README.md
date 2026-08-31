# HarmonicScope-WASM (GridHarmonics-Core)

Enterprise-grade, ultra-fast Power Electronics & Inverter Harmonic Distortion (THD) Simulator for WebAssembly and Edge devices. 
Designed for solar/wind inverter manufacturers, BESS grid-tie engineers, and active power filter (APF/STATCOM) designers to execute sub-millisecond Fast Fourier Transform (FFT) spectral decomposition and verify IEEE 519 / IEC 61000-4-7 THD compliance.

## Dual Licensing Model (Open-Core)
HarmonicScope-WASM follows an open-core dual-licensing model:
* **Community Edition (AGPLv3):** Free and open source for hobbyists, academia, and non-commercial open-source applications. Includes standard FFT up to 25th harmonic.
* **Enterprise Pro Tier (Proprietary B2B License):** Built for OEMs, hardware-in-the-loop (HIL) automation, and commercial test-bench developers. Supports three-phase THD evaluation, up to 50th harmonic limits, dynamic STATCOM/APF reactive power simulation, and works 100% offline using a cryptographic Ed25519 licensing model.

**[Purchase Enterprise Pro via Polar.sh](https://buy.polar.sh/polar_cl_o5aMJ1RP2l5qESYsrlBT1PhZCnt1q3Zntnz3y4GuBbA) **

---

## Performance Benchmarks 🚀

| Operation | Environment | Execution Time | Max Throughput |
|-----------|-------------|----------------|----------------|
| 1-Phase FFT (1024 bins) | Native (C-ABI) | **<150 μs** | ~6,500 Hz |
| 3-Phase IEEE 519 Check | Native (C-ABI) | **<450 μs** | ~2,200 Hz |
| Waterfall Spectrum Render | WebGPU (WASM) | **60 FPS** | Zero-latency |

*Zero dynamic memory allocation during real-time telemetry sampling to prevent GC pauses and memory leaks.*

---

## Community vs. Enterprise Edition Feature Matrix

| Feature | Community Edition (AGPLv3) | Enterprise Pro (Proprietary) |
|---------|---------------------------|-----------------------------|
| Fast Fourier Transform (FFT) | Single-Phase | Three-Phase |
| Max Harmonic Order | 25th | 50th + Inter-harmonics |
| Compliance Standard | Standard THD formula | IEEE 519-2022 / IEC 61000-4-7 |
| STATCOM / APF Simulator | ❌ | ✅ |
| HIL Native C-ABI Bindings | ❌ | ✅ (Typhoon, Opal-RT, RTDS) |
| Offline License Verification | ❌ | ✅ (Ed25519 Cryptographic) |
| Support | Community (GitHub Issues) | Dedicated SLA & Setup Consult |

---

## ⚡ 3-Line Developer Quickstart

To ingest waveform array, compute standard harmonic FFT, and evaluate IEEE 519 pass/fail using the Enterprise Edition:

```rust
use enterprise::EnterpriseAnalyzer;

let mut analyzer = EnterpriseAnalyzer::new();
// 1. Verify your offline B2B license key (Ed25519 Signature)
analyzer.verify_license("PUB_KEY_HEX", b"MACHINE_ID_123", "SIGNATURE_HEX").unwrap();

// 2. Perform Three-Phase THD evaluation (IEEE 519-2022) on incoming buffers
let (thd_a, thd_b, thd_c) = analyzer.compute_three_phase_thd(&phase_a_buf, &phase_b_buf, &phase_c_buf).unwrap();

// 3. Verify Pass/Fail limits (e.g. < 5.0% THD limit)
assert!(thd_a < 5.0 && thd_b < 5.0 && thd_c < 5.0, "IEEE 519 Compliance Failed!");
```

---

## Licensing Hygiene & Source Separation

The repository is strictly separated to prevent GPL leakage into proprietary B2B environments:
- `/community`: Contains the AGPLv3 core. It has its own isolated compilation target.
- `/enterprise`: Contains the Pro modules, reactive power simulations, and license validators. This directory is stripped out in the public GitHub repository and is provided directly to Pro customers via Polar.sh as a private registry crate or binary SDK.

## Authors
**Emirhan CAMCI** <byemir@live.com> (c) 2026
