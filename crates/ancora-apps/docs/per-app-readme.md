# Per-App README and Architecture

## Document QA (`document_qa`)

Ingests documents into a local `DocumentStore` and answers queries by
scanning content for matching terms. No embedding model or vector DB is
required for the offline variant.

Architecture:
```
DocumentStore -> DocumentQaEngine -> Answer
```

## Research Assistant (`research_assistant`)

Maintains a `KnowledgeBase` of topic entries. Research queries are matched
by exact topic name or by tag membership.

Architecture:
```
KnowledgeBase -> ResearchAssistant -> ResearchSummary
```

## Coding Assistant (`coding_assistant`)

Provides a `SnippetLibrary` with per-language code snippets. Generates
boilerplate stubs without external tools.

Architecture:
```
SnippetLibrary -> CodingAssistant -> CodeSuggestion
```

## Data Analysis (`data_analysis`)

Operates over an in-memory `DataSet`. Computes descriptive statistics and
top-N row selection using only `std`.

Architecture:
```
DataSet -> DataAnalyzer -> ColumnStats
```

## Customer Support (`customer_support`)

Routes tickets against `ResponseTemplate` keyword patterns. Maintains
ticket lifecycle state (Open, InProgress, Resolved, Closed).

Architecture:
```
Ticket -> SupportEngine -> auto-response String
```

## Compliance Review - Government (`compliance_review`)

Evaluates artifact text against a set of `ComplianceRule` objects.
The `government_preset()` constructor provides GOV-001 through GOV-004 rules.
Findings are classified by severity; Critical or High findings fail the review.

Architecture:
```
Artifact text -> ComplianceReviewer -> ReviewResult { findings, passed }
```
