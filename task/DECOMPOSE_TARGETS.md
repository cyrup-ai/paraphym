# Decompose Targets

Files in `packages/candle/src/**/*.rs` with >= 500 lines of code

Ranked by most lines to least lines:

**Total files: 71**

| Rank | File | Lines |
|------|------|-------|
| 1 | `packages/candle/src/memory/core/manager/surreal.rs` | 2,062 |
| 2 | `packages/candle/src/domain/chat/macros.rs` | 2,032 |
| 3 | `packages/candle/src/domain/memory/cognitive/types.rs` | 1,970 |
| 4 | `packages/candle/src/domain/context/provider.rs` | 1,834 |
| 5 | `packages/candle/src/domain/chat/config.rs` | 1,469 |
| 6 | `packages/candle/src/memory/core/manager/coordinator.rs` | 1,330 |
| 7 | `packages/candle/src/domain/chat/commands/types/mod.rs` | 1,308 |
| 8 | `packages/candle/src/domain/chat/commands/types/code_execution.rs` | 1,273 |
| 9 | `packages/candle/src/domain/chat/formatting.rs` | 1,226 |
| 10 | `packages/candle/src/domain/context/chunk.rs` | 1,116 |
| 11 | `packages/candle/src/domain/error.rs` | 1,061 |
| 12 | `packages/candle/src/capability/vision/llava.rs` | 1,028 |
| 13 | `packages/candle/src/capability/image_embedding/clip_vision.rs` | 1,011 |
| 14 | `packages/candle/src/memory/core/cognitive_worker.rs` | 1,006 |
| 15 | `packages/candle/src/domain/chat/templates/parser.rs` | 942 |
| 16 | `packages/candle/src/domain/memory/config/vector.rs` | 937 |
| 17 | `packages/candle/src/builders/document.rs` | 915 |
| 18 | `packages/candle/src/memory/graph/entity.rs` | 909 |
| 19 | `packages/candle/src/memory/vector/vector_search.rs` | 907 |
| 20 | `packages/candle/src/domain/chat/commands/parsing.rs` | 894 |
| 21 | `packages/candle/src/domain/chat/commands/validation.rs` | 890 |
| 22 | `packages/candle/src/capability/text_embedding/stella.rs` | 883 |
| 23 | `packages/candle/src/domain/chat/templates/core.rs` | 878 |
| 24 | `packages/candle/src/domain/chat/realtime/streaming.rs` | 829 |
| 25 | `packages/candle/src/domain/chat/commands/types/commands.rs` | 818 |
| 26 | `packages/candle/src/domain/chat/commands/types/actions.rs` | 812 |
| 27 | `packages/candle/src/builders/agent_role/chat.rs` | 807 |
| 28 | `packages/candle/src/memory/monitoring/mod.rs` | 799 |
| 29 | `packages/candle/src/domain/chat/commands/types/events.rs` | 762 |
| 30 | `packages/candle/src/builders/image.rs` | 746 |
| 31 | `packages/candle/src/memory/core/ops/retrieval.rs` | 735 |
| 32 | `packages/candle/src/capability/text_embedding/nvembed.rs` | 715 |
| 33 | `packages/candle/src/capability/text_embedding/gte_qwen.rs` | 711 |
| 34 | `packages/candle/src/domain/memory/primitives/node.rs` | 709 |
| 35 | `packages/candle/src/core/generation/models.rs` | 688 |
| 36 | `packages/candle/src/capability/text_embedding/jina_bert.rs` | 688 |
| 37 | `packages/candle/src/capability/text_to_text/kimi_k2.rs` | 685 |
| 38 | `packages/candle/src/capability/text_embedding/bert.rs` | 679 |
| 39 | `packages/candle/src/capability/registry/pool/capabilities/image_embedding.rs` | 677 |
| 40 | `packages/candle/src/domain/chat/commands/execution.rs` | 674 |
| 41 | `packages/candle/src/memory/vector/vector_index.rs` | 672 |
| 42 | `packages/candle/src/capability/registry/pool/core/memory_governor.rs` | 671 |
| 43 | `packages/candle/src/domain/memory/primitives/types.rs` | 666 |
| 44 | `packages/candle/src/capability/text_to_image/flux_schnell.rs` | 651 |
| 45 | `packages/candle/src/capability/text_to_text/phi4_reasoning.rs` | 609 |
| 46 | `packages/candle/src/domain/chat/commands/types/parameters.rs` | 608 |
| 47 | `packages/candle/src/memory/core/systems/semantic.rs` | 600 |
| 48 | `packages/candle/src/domain/chat/commands/response.rs` | 600 |
| 49 | `packages/candle/src/memory/core/primitives/types.rs` | 599 |
| 50 | `packages/candle/src/memory/core/systems/history.rs` | 599 |
| 51 | `packages/candle/src/memory/core/systems/procedural.rs` | 594 |
| 52 | `packages/candle/src/domain/agent/chat.rs` | 593 |
| 53 | `packages/candle/src/domain/chat/message/mod.rs` | 587 |
| 54 | `packages/candle/src/memory/transaction/transaction_manager.rs` | 583 |
| 55 | `packages/candle/src/memory/migration/schema_migrations.rs` | 581 |
| 56 | `packages/candle/src/memory/core/ops/evolution.rs` | 572 |
| 57 | `packages/candle/src/domain/chat/realtime/events.rs` | 571 |
| 58 | `packages/candle/src/domain/chat/commands/types/metadata.rs` | 559 |
| 59 | `packages/candle/src/capability/registry/pool/capabilities/vision.rs` | 555 |
| 60 | `packages/candle/src/memory/migration/converter.rs` | 553 |
| 61 | `packages/candle/src/memory/monitoring/health.rs` | 547 |
| 62 | `packages/candle/src/domain/chat/realtime/typing.rs` | 543 |
| 63 | `packages/candle/src/capability/text_to_text/qwen3_quantized.rs` | 540 |
| 64 | `packages/candle/src/domain/memory/config/database.rs` | 537 |
| 65 | `packages/candle/src/capability/registry/pool/core/types.rs` | 527 |
| 66 | `packages/candle/src/domain/model/info.rs` | 518 |
| 67 | `packages/candle/src/core/tokenizer/core.rs` | 506 |
| 68 | `packages/candle/src/domain/chat/conversation/mod.rs` | 506 |
| 69 | `packages/candle/src/domain/agent/core.rs` | 504 |
| 70 | `packages/candle/src/domain/util/json_util.rs` | 503 |
| 71 | `packages/candle/src/capability/text_to_image/stable_diffusion_35_turbo/worker.rs` | 501 |
