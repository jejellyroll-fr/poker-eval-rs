use crate::tables::rank_lookup;
use std::time::{Duration, Instant};
use wgpu::util::DeviceExt;

/// Throughput metrics returned by streaming GPU evaluation runs.
#[derive(Debug, Clone)]
pub struct GPUStreamingStats {
    pub total_hands: u64,
    pub chunk_size: usize,
    pub chunks_processed: u64,
    pub elapsed: Duration,
    pub hands_per_second: f64,
    pub checksum: u64,
}

/// GPU-accelerated hand evaluator using wgpu.
pub struct GPUEvaluator {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    tables_bind_group: wgpu::BindGroup,
    input_bind_group_layout: wgpu::BindGroupLayout,
    buffer_capacity: usize,
    input_buf: Option<wgpu::Buffer>,
    output_buf: Option<wgpu::Buffer>,
    readback_buf: Option<wgpu::Buffer>,
    resident_pending_count: Option<usize>,
}

impl GPUEvaluator {
    /// Initializes the GPU evaluator and uploads the lookup tables.
    pub async fn new() -> Option<Self> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Poker Evaluator Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .ok()?;

        // 1. Create Lookup Table Buffers
        let noflush_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("No-Flush Lookup"),
            contents: bytemuck::cast_slice(&rank_lookup::NOFLUSH_LOOKUP),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let flush_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Flush Lookup"),
            contents: bytemuck::cast_slice(&rank_lookup::FLUSH_LOOKUP),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let row_offsets_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Row Offsets"),
            contents: bytemuck::cast_slice(&rank_lookup::PERF_HASH_ROW_OFFSETS),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let suit_hash_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Suit Hash"),
            contents: bytemuck::cast_slice(&rank_lookup::SUIT_HASH),
            usage: wgpu::BufferUsages::STORAGE,
        });

        // 2. Bind Group Layouts
        let tables_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Tables BGL"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let tables_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Tables BG"),
            layout: &tables_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: noflush_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: flush_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: row_offsets_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: suit_hash_buf.as_entire_binding(),
                },
            ],
        });

        let input_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Input BGL"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // 3. Pipeline Setup
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Eval Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("eval.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Eval Pipeline Layout"),
            bind_group_layouts: &[&tables_bgl, &input_bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Eval Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        Some(Self {
            device,
            queue,
            pipeline,
            tables_bind_group: tables_bg,
            input_bind_group_layout: input_bgl,
            buffer_capacity: 0,
            input_buf: None,
            output_buf: None,
            readback_buf: None,
            resident_pending_count: None,
        })
    }

    fn ensure_capacity(&mut self, hand_count: usize) {
        if hand_count <= self.buffer_capacity {
            return;
        }

        let new_capacity = hand_count.next_power_of_two();
        let input_size = (new_capacity as u64) * 8;
        let output_size = (new_capacity as u64) * 4;

        self.input_buf = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Hand Inputs"),
            size: input_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));

        self.output_buf = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Hand Results"),
            size: output_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        }));

        self.readback_buf = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback"),
            size: output_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));

        self.buffer_capacity = new_capacity;
    }

    fn dispatch_compute(&mut self, masks: &[u64]) {
        let input_buf = self
            .input_buf
            .as_ref()
            .expect("input buffer must be allocated");
        let output_buf = self
            .output_buf
            .as_ref()
            .expect("output buffer must be allocated");
        self.queue
            .write_buffer(input_buf, 0, bytemuck::cast_slice(masks));

        let input_bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Input BG"),
            layout: &self.input_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buf.as_entire_binding(),
                },
            ],
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, &self.tables_bind_group, &[]);
            cpass.set_bind_group(1, &input_bg, &[]);
            let group_count = (masks.len() as u32 + 63) / 64;
            cpass.dispatch_workgroups(group_count, 1, 1);
        }
        self.queue.submit(Some(encoder.finish()));
    }

    fn readback_results(&mut self, hand_count: usize) -> Vec<u32> {
        let output_size = (hand_count * 4) as u64;
        let output_buf = self
            .output_buf
            .as_ref()
            .expect("output buffer must be allocated");
        let readback_buf = self
            .readback_buf
            .as_ref()
            .expect("readback buffer must be allocated");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(output_buf, 0, readback_buf, 0, output_size);
        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = readback_buf.slice(0..output_size);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        self.device.poll(wgpu::Maintain::Wait);

        if receiver.recv().unwrap().is_ok() {
            let data = buffer_slice.get_mapped_range();
            let result = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            readback_buf.unmap();
            result
        } else {
            vec![0; hand_count]
        }
    }

    /// Dispatches a GPU batch and keeps results resident on device memory until collected.
    pub fn dispatch_batch_resident(&mut self, masks: &[u64]) {
        if masks.is_empty() {
            self.resident_pending_count = Some(0);
            return;
        }
        self.ensure_capacity(masks.len());
        self.dispatch_compute(masks);
        self.resident_pending_count = Some(masks.len());
    }

    /// Returns true if a resident batch has been dispatched and not read back yet.
    pub fn has_pending_resident_results(&self) -> bool {
        self.resident_pending_count.is_some()
    }

    /// Reads back the most recent resident batch.
    pub fn collect_resident_results(&mut self) -> Option<Vec<u32>> {
        let hand_count = self.resident_pending_count.take()?;
        if hand_count == 0 {
            return Some(Vec::new());
        }
        Some(self.readback_results(hand_count))
    }

    /// Evaluates a batch of hand masks on the GPU.
    pub fn evaluate_batch(&mut self, masks: &[u64]) -> Vec<u32> {
        if masks.is_empty() {
            return Vec::new();
        }

        self.ensure_capacity(masks.len());
        self.dispatch_compute(masks);
        self.readback_results(masks.len())
    }

    /// Evaluates a potentially large batch by splitting it into fixed-size chunks.
    pub fn evaluate_batch_chunked(&mut self, masks: &[u64], chunk_size: usize) -> Vec<u32> {
        if masks.is_empty() {
            return Vec::new();
        }
        let chunk_size = chunk_size.max(1);
        let mut out = Vec::with_capacity(masks.len());
        for chunk in masks.chunks(chunk_size) {
            out.extend(self.evaluate_batch(chunk));
        }
        out
    }

    /// Selects the fastest chunk size from candidates on a representative sample.
    pub fn autotune_chunk_size(&mut self, sample_masks: &[u64], candidates: &[usize]) -> usize {
        if sample_masks.is_empty() {
            return 1;
        }
        let mut best = sample_masks.len().clamp(1, 8192);
        let mut best_elapsed = Duration::MAX;
        for &candidate in candidates {
            let chunk_size = candidate.max(1);
            let start = Instant::now();
            let _ = self.evaluate_batch_chunked(sample_masks, chunk_size);
            let elapsed = start.elapsed();
            if elapsed < best_elapsed {
                best_elapsed = elapsed;
                best = chunk_size;
            }
        }
        best
    }

    /// Streams evaluation over a large workload and returns throughput metrics.
    pub fn stream_evaluate<F>(
        &mut self,
        total_hands: u64,
        chunk_size: usize,
        mut next_chunk: F,
    ) -> GPUStreamingStats
    where
        F: FnMut(usize) -> Vec<u64>,
    {
        let chunk_size = chunk_size.max(1);
        let mut processed = 0u64;
        let mut chunks = 0u64;
        let mut checksum = 0u64;
        let start = Instant::now();

        while processed < total_hands {
            let remaining = total_hands - processed;
            let requested = remaining.min(chunk_size as u64) as usize;
            let masks = next_chunk(requested);
            if masks.is_empty() {
                break;
            }
            let results = self.evaluate_batch(&masks);
            for value in &results {
                checksum = checksum.wrapping_add(*value as u64);
            }
            processed += masks.len() as u64;
            chunks += 1;
        }

        let elapsed = start.elapsed();
        let hands_per_second = if elapsed.as_secs_f64() > 0.0 {
            processed as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        GPUStreamingStats {
            total_hands: processed,
            chunk_size,
            chunks_processed: chunks,
            elapsed,
            hands_per_second,
            checksum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::StdDeck;
    use crate::evaluators::HandEvaluator;

    #[test]
    fn test_gpu_eval_vs_cpu() {
        pollster::block_on(async {
            let mut gpu = match GPUEvaluator::new().await {
                Some(g) => g,
                None => {
                    println!("Skipping GPU test: No compatible adapter found.");
                    return;
                }
            };

            let hands = vec![
                "AsKsQsJsTs5h2d", // Royal Flush
                "AhKhQhJh9h5h2d", // Flush
                "AsAhAdKsKh5h2d", // Full House
                "2s3s4s5s7h8h9h", // No Pair
            ];

            let mut masks = Vec::new();
            let mut cpu_results = Vec::new();
            for h_str in hands {
                let (mask, _) = StdDeck::string_to_mask(h_str).unwrap();
                masks.push(mask.as_raw());

                // CPU result (Holdem style: 2+5)
                // But StdDeck bitmask can be evaluated directly by Eval::eval_n
                let val = crate::evaluators::HoldemEvaluator::evaluate_hand(
                    &mask,
                    &crate::deck::StdDeckCardMask::new(),
                )
                .unwrap();
                cpu_results.push(val.value);
            }

            let gpu_results = gpu.evaluate_batch(&masks);

            for (i, cpu) in cpu_results.iter().enumerate() {
                assert_eq!(
                    *cpu, gpu_results[i],
                    "GPU result for hand {} does not match CPU",
                    i
                );
            }
            println!("GPU results match CPU results perfectly!");
        });
    }

    #[test]
    fn test_gpu_chunked_matches_batch() {
        pollster::block_on(async {
            let mut gpu = match GPUEvaluator::new().await {
                Some(g) => g,
                None => return,
            };

            let hands = vec![
                "AsKsQsJsTs5h2d",
                "AhKhQhJh9h5h2d",
                "AsAhAdKsKh5h2d",
                "2s3s4s5s7h8h9h",
                "AcKcQcJc9c8c7d",
                "AdKdQdJdTd9d8s",
                "2c3d4h5s6c7d8h",
                "TsThTdTc9s8h7d",
            ];
            let mut masks = Vec::with_capacity(hands.len());
            for h_str in hands {
                let (mask, _) = StdDeck::string_to_mask(h_str).unwrap();
                masks.push(mask.as_raw());
            }

            let full = gpu.evaluate_batch(&masks);
            let chunked = gpu.evaluate_batch_chunked(&masks, 3);
            assert_eq!(full, chunked);
        });
    }

    #[test]
    fn test_gpu_resident_matches_immediate() {
        pollster::block_on(async {
            let mut gpu = match GPUEvaluator::new().await {
                Some(g) => g,
                None => return,
            };
            let hands = vec![
                "AsKsQsJsTs5h2d",
                "AhKhQhJh9h5h2d",
                "AsAhAdKsKh5h2d",
                "2s3s4s5s7h8h9h",
            ];
            let mut masks = Vec::with_capacity(hands.len());
            for h_str in hands {
                let (mask, _) = StdDeck::string_to_mask(h_str).unwrap();
                masks.push(mask.as_raw());
            }

            let immediate = gpu.evaluate_batch(&masks);
            gpu.dispatch_batch_resident(&masks);
            assert!(gpu.has_pending_resident_results());
            let resident = gpu.collect_resident_results().unwrap();
            assert_eq!(immediate, resident);
            assert!(!gpu.has_pending_resident_results());
        });
    }
}
