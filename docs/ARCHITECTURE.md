# SoroProtocol Architecture

## Contract Overview

```
┌───────────────────────────────────────────────┐
│             SoroProtocol Contracts             │
│                                               │
│  ┌──────────┐  ┌──────────┐  ┌─────────────┐ │
│  │  stream  │  │ vesting  │  │ distributor │ │
│  └──────────┘  └──────────┘  └─────────────┘ │
└───────────────────────────────────────────────┘
```

## Stream Contract

Holds deposited tokens in escrow. Releases them linearly second-by-second.

**Key design decisions:**
- Deposit is calculated upfront: `rate * (stop - start)`
- Withdrawal is pull-based: recipient calls `withdraw()` at any time
- Cancellation is atomic: recipient gets accrued portion, sender gets refund

**Storage layout:**

| Key         | Scope      | Description              |
|-------------|------------|--------------------------|
| `next_id`   | Instance   | Monotonic stream counter |
| `stream_id` | Persistent | Stream struct per ID     |

## Vesting Contract

Linear release schedule with optional cliff. No tokens flow before `cliff_time`.
After cliff, tokens vest linearly from `cliff_time` to `end_time`.

## Distributor Contract

Batch-creates N streams in one transaction. Reduces ledger overhead for payroll
and grant distribution use cases.
