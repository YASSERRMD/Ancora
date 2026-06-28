# App Gallery Index

## Documentation

- [Overview](overview.md) - design principles and app table
- [Per-app README and architecture](per-app-readme.md) - each app's structure
- [Deploying a sample app](deploying.md) - build, run, air-gapped deployment
- [Adapting an app](adapting.md) - customisation guide
- [CI smoke tests](ci.md) - GitHub Actions workflow and test coverage table

## API Entry Points

| Module | Key Types | Entry Function |
|--------|-----------|---------------|
| `document_qa` | `DocumentStore`, `DocumentQaEngine` | `engine.ask(query)` |
| `research_assistant` | `KnowledgeBase`, `ResearchAssistant` | `ra.research(topic)` |
| `coding_assistant` | `SnippetLibrary`, `CodingAssistant` | `ca.suggest(query, lang)` |
| `data_analysis` | `DataSet`, `DataAnalyzer` | `DataAnalyzer::summarise(ds, col)` |
| `customer_support` | `SupportEngine` | `engine.auto_respond(ticket_id)` |
| `compliance_review` | `ComplianceReviewer` | `reviewer.review(id, text)` |
| `local_models` | `ModelRegistry`, `ModelDescriptor` | `run_local_inference(model, prompt)` |
| `traces` | `Tracer`, `Span` | `tracer.record(span)` |
| `safety` | `SafetyGuardrail` | `guard.evaluate(text)` |
| `index` | `AppEntry` | `gallery()`, `air_gapped_apps()` |
