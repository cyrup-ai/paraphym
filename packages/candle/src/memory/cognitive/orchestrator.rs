// src/cognitive/orchestrator.rs
//! Infinite agentic orchestrator for committee-driven optimization

use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};
use walkdir::WalkDir;

use crate::cognitive::evolution::{CodeEvolution, CognitiveCodeEvolution};
use crate::cognitive::types::{CognitiveError, OptimizationOutcome, OptimizationSpec};

/// Orchestrator managing infinite optimization iterations
pub struct InfiniteOrchestrator {
    spec_file: PathBuf,
    output_dir: PathBuf,
    spec: Arc<OptimizationSpec>,
    user_objective: String,
    initial_code: String,
    initial_latency: f64,
    initial_memory: f64,
    initial_relevance: f64,
}

impl InfiniteOrchestrator {
    pub fn new<P: AsRef<Path>>(
        spec_file: P,
        output_dir: P,
        initial_code: String,
        initial_latency: f64,
        initial_memory: f64,
        initial_relevance: f64,
        user_objective: String,
    ) -> Result<Self, CognitiveError> {
        let spec_file = spec_file.as_ref().to_path_buf();
        let output_dir = output_dir.as_ref().to_path_buf();
        let spec = Self::parse_spec(&spec_file)?;

        fs::create_dir_all(&output_dir)
            .map_err(|e| CognitiveError::OrchestrationError(e.to_string()))?;

        Ok(Self {
            spec_file,
            output_dir,
            spec: Arc::new(spec),
            user_objective,
            initial_code,
            initial_latency,
            initial_memory,
            initial_relevance,
        })
    }

    fn parse_spec<P: AsRef<Path>>(spec_file: P) -> Result<OptimizationSpec, CognitiveError> {
        let mut file =
            File::open(spec_file).map_err(|e| CognitiveError::SpecError(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| CognitiveError::SpecError(e.to_string()))?;

        // Try to parse as JSON first
        if let Ok(spec) = serde_json::from_str(&contents) {
            return Ok(spec);
        }

        // Otherwise convert Markdown to spec
        markdown_to_spec(&contents)
    }

    /// Reload specification from file (useful for runtime config changes)
    pub fn reload_spec(&mut self) -> Result<(), CognitiveError> {
        let new_spec = Self::parse_spec(&self.spec_file)?;
        self.spec = Arc::new(new_spec);
        tracing::info!("Reloaded specification from {:?}", self.spec_file);
        Ok(())
    }

    /// Get the current spec file path for debugging/logging
    pub fn spec_file_path(&self) -> &Path {
        &self.spec_file
    }

    fn scan_output_dir(&self) -> Result<(u64, Vec<(PathBuf, u64)>, Vec<String>), CognitiveError> {
        let mut files = vec![];
        let mut max_iter = 0;
        let mut gaps = vec![];

        for entry in WalkDir::new(&self.output_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && entry.file_name().to_string_lossy().ends_with(".json")
            {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with("iteration_") {
                        if let Some(iter_str) = file_name
                            .strip_prefix("iteration_")
                            .and_then(|s| s.strip_suffix(".json"))
                        {
                            if let Ok(iter) = iter_str.parse::<u64>() {
                                files.push((entry.path().to_path_buf(), iter));
                                max_iter = max_iter.max(iter);
                            }
                        }
                    }
                }
            }
        }

        // Check for missing iterations
        for i in 1..=max_iter {
            if !files.iter().any(|(_, iter)| *iter == i) {
                gaps.push(format!("Missing iteration {}", i));
            }
        }

        Ok((max_iter, files, gaps))
    }

    async fn create_evolution(
        &self,
        base_code: String,
        base_latency: f64,
        base_memory: f64,
        base_relevance: f64,
    ) -> Result<Arc<CognitiveCodeEvolution>, CognitiveError> {
        let evolution = CognitiveCodeEvolution::new(
            base_code,
            base_latency,
            base_memory,
            base_relevance,
            self.spec.clone(),
            self.user_objective.clone(),
        )?;

        Ok(Arc::new(evolution))
    }

    pub async fn run_infinite(&self) -> Result<(), CognitiveError> {
        let (max_iter, _, gaps) = self.scan_output_dir()?;
        let mut current_iter = max_iter + 1;
        let mut join_set: JoinSet<Result<OptimizationOutcome, CognitiveError>> = JoinSet::new();
        let mut outcomes: Vec<OptimizationOutcome> = vec![];

        if !gaps.is_empty() {
            warn!("Detected gaps in output: {:?}", gaps);
        }

        // Track current best state
        let best_code = self.initial_code.clone();
        let mut best_latency = self.initial_latency;
        let mut best_memory = self.initial_memory;
        let mut best_relevance = self.initial_relevance;

        loop {
            // Create iteration plan for this round
            let best_outcome = outcomes
                .iter()
                .filter(|o| o.applied())
                .max_by(|a, b| {
                    let a_score = match a {
                        OptimizationOutcome::Success {
                            performance_gain,
                            quality_score,
                            ..
                        } => *performance_gain as f64 + *quality_score as f64,
                        OptimizationOutcome::PartialSuccess {
                            performance_gain,
                            quality_score,
                            ..
                        } => (*performance_gain as f64 + *quality_score as f64) * 0.5,
                        _ => 0.0,
                    };
                    let b_score = match b {
                        OptimizationOutcome::Success {
                            performance_gain,
                            quality_score,
                            ..
                        } => *performance_gain as f64 + *quality_score as f64,
                        OptimizationOutcome::PartialSuccess {
                            performance_gain,
                            quality_score,
                            ..
                        } => (*performance_gain as f64 + *quality_score as f64) * 0.5,
                        _ => 0.0,
                    };
                    a_score
                        .partial_cmp(&b_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned();

            let iteration_plan = IterationPlan::new(current_iter, best_outcome);

            // Log iteration plan details for monitoring
            if let Some(performance_gain) = iteration_plan.get_performance_gain() {
                tracing::debug!(
                    "Iteration {} planned with performance gain: {:.2}%",
                    iteration_plan.iteration(),
                    performance_gain
                );
            }

            if let Some(quality_score) = iteration_plan.get_quality_score() {
                tracing::debug!(
                    "Iteration {} quality score: {:.2}",
                    iteration_plan.iteration(),
                    quality_score
                );
            }

            if let Some(improvements) = iteration_plan.get_improvements() {
                if !improvements.is_empty() {
                    tracing::debug!(
                        "Iteration {} improvements: {:?}",
                        iteration_plan.iteration(),
                        improvements
                    );
                }
            }

            // Adaptive agent count based on recent success
            let agents_per_wave =
                if outcomes.len() > 10 && outcomes.iter().rev().take(5).all(|o| !o.applied()) {
                    3 // Scale down if no recent progress
                } else {
                    5 // Default for infinite mode
                };

            // Wait if we have too many concurrent tasks
            while join_set.len() >= agents_per_wave {
                if let Some(res) = join_set.join_next().await {
                    match res {
                        Ok(Ok(outcome)) => {
                            // Update best state if improved
                            if outcome.applied() {
                                match &outcome {
                                    OptimizationOutcome::Success {
                                        performance_gain,
                                        quality_score,
                                        ..
                                    } => {
                                        // Use performance_gain and quality_score to update metrics
                                        let improvement_factor =
                                            (*performance_gain as f64).max(0.0) / 100.0;
                                        let quality_factor =
                                            (*quality_score as f64).max(0.0) / 100.0;

                                        best_latency =
                                            best_latency * (1.0 - improvement_factor * 0.1);
                                        best_memory =
                                            best_memory * (1.0 - improvement_factor * 0.1);
                                        best_relevance =
                                            best_relevance * (1.0 + quality_factor * 0.1);

                                        info!(
                                            "New best state: latency={:.2}, memory={:.2}, relevance={:.2}",
                                            best_latency, best_memory, best_relevance
                                        );
                                    }
                                    OptimizationOutcome::PartialSuccess {
                                        performance_gain,
                                        quality_score,
                                        ..
                                    } => {
                                        // Partial improvements are smaller
                                        let improvement_factor =
                                            (*performance_gain as f64).max(0.0) / 200.0;
                                        let quality_factor =
                                            (*quality_score as f64).max(0.0) / 200.0;

                                        best_latency =
                                            best_latency * (1.0 - improvement_factor * 0.05);
                                        best_memory =
                                            best_memory * (1.0 - improvement_factor * 0.05);
                                        best_relevance =
                                            best_relevance * (1.0 + quality_factor * 0.05);
                                    }
                                    OptimizationOutcome::Failure { .. } => {
                                        // No improvements for failures
                                    }
                                }
                            }

                            outcomes.push(outcome.clone());

                            // Save outcome
                            let output_path = self
                                .output_dir
                                .join(format!("iteration_{}.json", current_iter));
                            fs::write(&output_path, serde_json::to_string_pretty(&outcome)?)
                                .map_err(|e| CognitiveError::OrchestrationError(e.to_string()))?;

                            info!("Saved outcome for iteration {}", current_iter);
                        }
                        Ok(Err(e)) => error!("Evolution task failed: {}", e),
                        Err(e) => error!("Evolution task panicked: {}", e),
                    }
                    current_iter += 1;
                }
            }

            // Create evolution with current best state
            let evolution = self
                .create_evolution(best_code.clone(), best_latency, best_memory, best_relevance)
                .await?;

            // Spawn optimization task
            join_set.spawn(async move { evolution.evolve_routing_logic().await });

            // Brief pause to prevent CPU saturation
            sleep(Duration::from_millis(100)).await;

            // Log progress periodically
            if current_iter % 10 == 0 {
                let successful = outcomes.iter().filter(|o| o.applied()).count();
                info!(
                    "Progress: {} iterations, {} successful optimizations",
                    current_iter, successful
                );
            }
        }
    }
}

/// Convert Markdown spec to OptimizationSpec
fn markdown_to_spec(md: &str) -> Result<OptimizationSpec, CognitiveError> {
    use crate::cognitive::types::*;

    // Default values
    let mut max_latency_increase = 20.0;
    let mut max_memory_increase = 30.0;
    let mut min_relevance_improvement = 40.0;
    let mut baseline_latency = 10.0;
    let mut baseline_memory = 100.0;
    let mut baseline_relevance = 50.0;

    // Parse markdown for key values
    for line in md.lines() {
        if line.contains("latency increase") {
            if let Some(num) = extract_percentage(line) {
                max_latency_increase = num;
            }
        } else if line.contains("memory increase") {
            if let Some(num) = extract_percentage(line) {
                max_memory_increase = num;
            }
        } else if line.contains("relevance improvement") {
            if let Some(num) = extract_percentage(line) {
                min_relevance_improvement = num;
            }
        } else if line.contains("Latency:") {
            if let Some(num) = extract_number(line) {
                baseline_latency = num;
            }
        } else if line.contains("Memory:") {
            if let Some(num) = extract_number(line) {
                baseline_memory = num;
            }
        } else if line.contains("Relevance:") {
            if let Some(num) = extract_percentage(line) {
                baseline_relevance = num;
            }
        }
    }

    Ok(OptimizationSpec {
        objective: "Improve code performance and quality".to_string(),
        improvement_threshold: 0.1, // 10% improvement threshold
        constraints: vec!["Memory efficient".to_string(), "Thread safe".to_string()],
        success_criteria: vec![
            "Performance improvement".to_string(),
            "Quality score > 0.8".to_string(),
        ],
        optimization_type: OptimizationType::Performance,
        timeout_ms: Some(30000),
        max_iterations: Some(10),
        target_quality: 0.8,
        content_type: ContentType {
            category: ContentCategory::Code,
            complexity: 0.7,
            processing_hints: vec!["Rust".to_string(), "Performance".to_string()],
            format: "Rust source code".to_string(),
            restrictions: Restrictions {
                max_memory_usage: Some(1024 * 1024 * 100), // 100MB
                max_processing_time: Some(30000),          // 30 seconds
                allowed_operations: vec!["optimization".to_string(), "refactoring".to_string()],
                forbidden_operations: vec!["unsafe".to_string(), "system_calls".to_string()],
                security_level: SecurityLevel::Internal,
                compiler: "rustc 1.82.0".to_string(),
                max_latency_increase,
                max_memory_increase,
                min_relevance_improvement,
            },
        },
        evolution_rules: EvolutionRules {
            mutation_rate: 0.1,
            selection_pressure: 0.8,
            crossover_rate: 0.7,
            elite_retention: 0.2,
            diversity_maintenance: 0.3,
            allowed_mutations: vec![
                MutationType::AttentionWeightAdjustment,
                MutationType::RoutingStrategyModification,
            ],
            build_on_previous: true,
            new_axis_per_iteration: true,
            max_cumulative_latency_increase: max_latency_increase,
            min_action_diversity: 30.0,
            validation_required: true,
        },
        baseline_metrics: BaselineMetrics {
            response_time: 1.0,
            accuracy: 0.9,
            throughput: 100.0,
            resource_usage: 0.5,
            error_rate: 0.05,
            quality_score: 0.8,
            latency: baseline_latency,
            memory: baseline_memory,
            relevance: baseline_relevance,
        },
    })
}

fn extract_percentage(line: &str) -> Option<f64> {
    line.split_whitespace()
        .find(|word| word.ends_with('%'))
        .and_then(|word| word.trim_end_matches('%').parse().ok())
}

fn extract_number(line: &str) -> Option<f64> {
    line.split_whitespace()
        .find_map(|word| word.parse::<f64>().ok())
}

#[derive(Debug)]
struct IterationPlan {
    iteration: u64,
    base_state: Option<OptimizationOutcome>,
}

impl IterationPlan {
    /// Create a new iteration plan
    fn new(iteration: u64, base_state: Option<OptimizationOutcome>) -> Self {
        Self {
            iteration,
            base_state,
        }
    }

    /// Get the iteration number
    fn iteration(&self) -> u64 {
        self.iteration
    }

    /// Get performance gain for this iteration
    fn get_performance_gain(&self) -> Option<f32> {
        self.base_state.as_ref().and_then(|state| match state {
            OptimizationOutcome::Success {
                performance_gain, ..
            } => Some(*performance_gain),
            OptimizationOutcome::PartialSuccess {
                performance_gain, ..
            } => Some(*performance_gain),
            OptimizationOutcome::Failure { .. } => None,
        })
    }

    /// Get quality score for this iteration
    #[allow(dead_code)]
    fn get_quality_score(&self) -> Option<f32> {
        self.base_state.as_ref().and_then(|state| match state {
            OptimizationOutcome::Success { quality_score, .. } => Some(*quality_score),
            OptimizationOutcome::PartialSuccess { quality_score, .. } => Some(*quality_score),
            OptimizationOutcome::Failure { .. } => None,
        })
    }

    /// Get improvement summary for this iteration
    #[allow(dead_code)]
    fn get_improvements(&self) -> Option<&Vec<String>> {
        self.base_state.as_ref().and_then(|state| match state {
            OptimizationOutcome::Success { improvements, .. } => Some(improvements),
            OptimizationOutcome::PartialSuccess { improvements, .. } => Some(improvements),
            OptimizationOutcome::Failure { .. } => None,
        })
    }
}
