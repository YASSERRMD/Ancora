# Choosing a Quantization Level

Quantization reduces model size and speeds up inference at the cost of output
quality. This guide explains the trade-offs so you can pick the right level
for your use case.

## Quick Decision Guide

| Scenario | Recommended Level | Reasoning |
|---|---|---|
| Research / reference | FP32 / FP16 | No quality loss |
| Production GPU server | Q8_0 or Q5_K_M | Near-lossless, fast |
| Consumer GPU (8-16 GB) | Q5_K_M or Q4_K_M | Good balance |
| CPU laptop (16 GB) | Q4_K_M | Fits comfortably |
| CPU laptop (8 GB) | Q4_K_S or Q3_K_M | Tighter fit |
| Edge / embedded | Q2_K | Minimum viable |

## Quantization Tiers

### Full Precision (FP32)

- Bits per weight: 32
- Compression ratio: 1x (baseline)
- Quality: reference quality
- Use case: research, fine-tuning, very high quality requirements
- Limitation: 28+ GB RAM for a 7B model; GPU required for practical speed

### Half Precision (FP16 / BF16)

- Bits per weight: 16
- Compression ratio: ~2x
- Quality: imperceptible difference from FP32
- Use case: GPU deployments with 14+ GB VRAM
- Limitation: CPU inference is slow

### INT8 / Q8_0

- Bits per weight: 8
- Compression ratio: ~4x
- Quality: near-lossless (<0.1 perplexity increase)
- Use case: best balance of quality and size on CPU servers
- RAM for 7B: ~7 GB

### Medium (Q5/Q6 K-Quants)

- Bits per weight: 5.0 to 6.5
- Compression ratio: ~5-6x
- Quality: small but noticeable quality loss (<0.2 perplexity increase)
- Use case: default recommendation for most deployments
- RAM for 7B: ~5 GB

### INT4 / Q4_K

- Bits per weight: ~4.5
- Compression ratio: ~8x
- Quality: noticeable quality loss (<0.4 perplexity increase)
- Use case: consumer hardware with limited RAM
- RAM for 7B: ~4 GB

### Aggressive (Q2/Q3 K-Quants)

- Bits per weight: 2.5 to 3.5
- Compression ratio: ~10-12x
- Quality: significant quality loss (>1.0 perplexity increase)
- Use case: embedded / IoT devices only; avoid for production
- RAM for 7B: ~2-3 GB

## K-Quants vs Legacy Quants

K-quants (Q4_K, Q5_K, Q6_K, etc.) use grouped / block quantization and are
generally superior to their legacy counterparts (Q4_0, Q5_0) of the same bit
width. Always prefer K-quants when available.

## Size Variants (S, M, L)

Many K-quant models come in S (small), M (medium), and L (large) variants
that vary which layers receive higher precision. M is the standard choice.

- `Q4_K_S`: smaller file, slightly lower quality
- `Q4_K_M`: balanced (recommended default)
- `Q4_K_L`: larger file, slightly better quality

## Estimating RAM Requirements

Use `GgufDescriptor::estimated_ram_bytes()` or `OnnxDescriptor::estimated_ram_bytes()`
for an estimate. These include a ~10% overhead for KV cache and buffers.

Or use the formula directly:

```
ram_gb = param_billions * bits_per_weight / 8 * 1.1
```

Example: 7B model at Q4_K (4.5 bpw):
```
7 * 4.5 / 8 * 1.1 = 4.3 GB
```

## Using `QuantLevel::recommend_for_ram_gb`

```rust
use ancora_quant::quant_level::QuantLevel;

let tier = QuantLevel::recommend_for_ram_gb(7.0 /* param_billions */, 8.0 /* ram_gb */);
println!("Recommended tier: {}", tier);
```

## Quality Validation

After selecting a quantization level, validate on a domain-specific benchmark:
- Use the same prompts / expected outputs as your production use case
- Compare perplexity on a representative text sample
- Test edge cases where model quality matters most (code, math, reasoning)
