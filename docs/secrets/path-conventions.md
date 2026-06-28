# Path Conventions

Secret paths follow a hierarchical naming scheme similar to a filesystem or vault path.

## Structure

```
{category}/{subcategory}/{name}
```

Examples:
- `database/prod/password`
- `api/stripe/key`
- `tls/ingress/cert`
- `ssh/deploy/private-key`
- `service/payment-processor/webhook-secret`

## Allowed characters

| Character class | Allowed |
|---|---|
| Lowercase letters `a-z` | Yes |
| Uppercase letters `A-Z` | Yes |
| Digits `0-9` | Yes |
| Slash `/` | Yes (as separator) |
| Dot `.` | Yes |
| Hyphen `-` | Yes |
| Underscore `_` | Yes |
| Space | No |
| Any other character | No |

## Rules

- Path must not be empty
- Path must not start or end with `/`
- Maximum 256 characters
- No consecutive slashes `//`

## Tenant isolation

The `SecretStore` internally keys secrets as `{tenant_id}:{path}`, so two tenants may independently store secrets at the same path without conflict. Never pass a path that embeds the tenant ID -- the tenant scope is always applied by the store methods.
