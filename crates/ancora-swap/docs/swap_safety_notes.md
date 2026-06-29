# swap safety notes

## atomicity

the swap itself (updating the active model pointer) is protected by a `Mutex`
so it is atomic with respect to concurrent `start_run` and `finish_run` calls.
no run can be started on a partially-swapped model.

## pin ordering

`start_run` acquires the lock, reads the active model, increments its pin
count, and stores the pin -- all under the same lock hold.  this prevents the
window where a swap could land between reading the handle and incrementing the
pin.

## warmup contract

callers must complete warmup before calling `swap`.  calling `swap` with an
unwarmed model is not prevented by the library (it is the caller's
responsibility) but will result in early runs on the new model incurring
warmup latency.

## rollback window

rollback is only possible while the old model is still held in the drain slot.
once `reclaim_unloaded` releases the drain slot (or `finish_run` auto-reclaims
it) rollback is no longer available.  if you need rollback support, keep a
reference to the old handle separately.

## memory reclaim timing

memory is not freed from OS until the `ModelHandle` (and all `ModelPin`s) are
dropped.  in this library that is simulated; in a real system you would
release GPU/CPU buffers in the handle's `Drop` implementation.

## journal replay

the journal records `(from, to)` version pairs.  replaying it requires having
access to the original model checkpoints.  the journal is not a full snapshot
of model weights.

## no concurrent swaps

calling `swap` while another swap's drain is still outstanding is allowed but
the previous drain model is overwritten.  ensure the prior drain has been
fully reclaimed before initiating a second swap if strict memory bounds are
required.
