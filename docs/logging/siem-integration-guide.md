# SIEM Integration Guide

## Overview

Ancora audit events can be exported in a SIEM-compatible JSON format. Each exported event includes CEF-inspired fields for ingestion by Splunk, QRadar, or Elastic SIEM.

## Export format

```rust
use ancora_logging::to_siem;

let siem_event = to_siem(&audit_event);
// siem_event is serde_json::Value with fields:
// DeviceVendor, DeviceProduct, DeviceVersion
// SignatureID (AuditEventKind), Name (decision)
// Extensions: suser (actor), dhost (resource), cs1 (tenant), rt (timestamp), sig (signature)
```

## Splunk integration

Configure a Splunk HTTP Event Collector (HEC) forwarder:

```json
{
  "index": "ancora_audit",
  "sourcetype": "_json",
  "event": <siem_export>
}
```

## Elastic integration

Index mapping for Elastic:

```json
{
  "mappings": {
    "properties": {
      "DeviceVendor": { "type": "keyword" },
      "SignatureID":  { "type": "keyword" },
      "Extensions": {
        "properties": {
          "suser": { "type": "keyword" },
          "cs1":   { "type": "keyword" },
          "rt":    { "type": "long" },
          "sig":   { "type": "keyword" }
        }
      }
    }
  }
}
```

## Signature verification

All exported events include a SHA-256 HMAC signature in `Extensions.sig`. Verify before ingesting:

```python
import hashlib, hmac

def verify(event, key):
    expected = hashlib.sha256(
        key + event['Extensions']['cs1'].encode() + ...
    ).hexdigest()
    return hmac.compare_digest(expected, event['Extensions']['sig'])
```

## Retention policy

Audit events must be retained for at minimum 90 days per most compliance frameworks. Configure your SIEM retention accordingly. Do not delete events from the AuditChannel in-process; archive to cold storage after 30 days.
