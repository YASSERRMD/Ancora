# RFC Process for Extension Points

RFCs (Requests for Comments) are the primary mechanism for proposing changes
to Ancora's extension API.

## When to Write an RFC

- Adding a new extension hook or lifecycle callback.
- Removing or deprecating an existing extension API point.
- Changing the version negotiation protocol.
- Modifying governance rules or security disclosure policy.

## RFC Lifecycle

1. **Draft**: Author opens a PR with the RFC document and registers it in the
   `RfcRegistry`.
2. **Final Comment Period (FCP)**: Core Maintainers announce the FCP. Community
   has at least 14 days to comment.
3. **Accepted / Rejected**: Core Maintainers vote and record the outcome as a
   `GovernanceDecision`.
4. **Implemented**: The RFC is implemented and the status updated to `Implemented`.
5. **Withdrawn**: The author may withdraw an RFC at any time before FCP ends.

## RFC Template

```
# RFC-NNN: Title

## Summary
One paragraph summary.

## Motivation
Why is this change needed?

## Detailed Design
Technical specification.

## Drawbacks
Known downsides.

## Alternatives
What other approaches were considered?

## Unresolved Questions
Open issues to resolve before stabilization.
```
