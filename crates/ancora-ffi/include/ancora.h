#ifndef ANCORA_H
#define ANCORA_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Return codes used by all extern "C" functions.
 */
typedef enum AncorErrorCode {
  Ok = 0,
  NullPtr = 1,
  InvalidUtf8 = 2,
  Internal = 3,
} AncorErrorCode;

/**
 * Owned byte buffer passed across the FFI boundary.
 * The caller is responsible for freeing with `ancora_buffer_free`.
 */
typedef struct AncorBuffer {
  uint8_t *ptr;
  uintptr_t len;
} AncorBuffer;

/**
 * Opaque handle to a single run identifier.
 */
typedef struct AncorRunId {
  uint8_t _private[0];
} AncorRunId;

/**
 * Opaque handle to a live Ancora runtime.
 */
typedef struct AncorRuntime {
  uint8_t _private[0];
} AncorRuntime;

/**
 * Host-provided tool callback. `input` contains the tool invocation payload as bytes.
 * The callback writes its output into `out` and returns an error code.
 */
typedef enum AncorErrorCode (*AncorToolCallback)(const uint8_t *input,
                                                 uintptr_t input_len,
                                                 struct AncorBuffer *out);

/**
 * Allocate a buffer containing a copy of `bytes`.
 * Returns a zero-length buffer with null ptr if `bytes` is empty.
 */
struct AncorBuffer ancora_buffer_new(const uint8_t *bytes, uintptr_t len);

/**
 * Free a buffer previously created by `ancora_buffer_new` or `ancora_buffer_from_str`.
 * Passing a zero-length or null-ptr buffer is a no-op.
 */
void ancora_buffer_free(struct AncorBuffer buf);

/**
 * Allocate a new run ID from a null-terminated UTF-8 string.
 * Returns null if `s` is null or not valid UTF-8.
 */
struct AncorRunId *ancora_run_id_new(const char *s);

/**
 * Free a run ID previously created by `ancora_run_id_new`.
 * Passing null is a no-op.
 */
void ancora_run_id_free(struct AncorRunId *ptr);

/**
 * Return the run ID string as an owned `AncorBuffer`.
 * The buffer must be freed with `ancora_buffer_free`.
 * Returns a zero-length buffer if `ptr` is null.
 */
struct AncorBuffer ancora_run_id_to_str(const struct AncorRunId *ptr);

/**
 * Start a new run from serialized agent spec bytes.
 * Writes the run ID (as UTF-8) into `out_run_id`.
 * Returns `NullPtr` if runtime or spec pointer is null.
 */
enum AncorErrorCode ancora_run_start(struct AncorRuntime *rt,
                                     const uint8_t *spec_bytes,
                                     uintptr_t spec_len,
                                     struct AncorBuffer *out_run_id);

/**
 * Poll the next event for a run. Writes event JSON bytes into `out_event`.
 * Returns an empty buffer in `out_event` when all events are consumed.
 * Returns `NullPtr` if any pointer is null.
 */
enum AncorErrorCode ancora_run_poll(struct AncorRuntime *rt,
                                    const char *run_id,
                                    struct AncorBuffer *out_event);

/**
 * Resume a suspended run by providing a decision as bytes.
 */
enum AncorErrorCode ancora_run_resume(struct AncorRuntime *rt,
                                      const char *run_id,
                                      const uint8_t *decision_bytes,
                                      uintptr_t decision_len);

/**
 * Return a JSON cost summary for a run as an AncorBuffer.
 * Returns `NullPtr` if any pointer is null.
 */
enum AncorErrorCode ancora_run_cost(struct AncorRuntime *rt,
                                    const char *run_id,
                                    struct AncorBuffer *out_cost);

/**
 * Allocate a new runtime. The caller owns the returned pointer.
 * Returns null on allocation failure.
 */
struct AncorRuntime *ancora_create_runtime(void);

/**
 * Allocate a runtime and write the pointer to `out`. Returns `NullPtr` if `out` is null.
 */
enum AncorErrorCode ancora_runtime_new(struct AncorRuntime **out);

/**
 * Allocate a runtime with serialized config bytes and write pointer to `out`.
 * Config bytes are currently ignored (reserved for future use).
 * Returns `NullPtr` if `out` is null.
 */
enum AncorErrorCode ancora_runtime_new_with_config(const uint8_t *_config_bytes,
                                                   uintptr_t _config_len,
                                                   struct AncorRuntime **out);

/**
 * Free a runtime previously created by `ancora_create_runtime`.
 * Passing null is a no-op.
 */
void ancora_free_runtime(struct AncorRuntime *ptr);

/**
 * Register a named tool callback on the runtime.
 * Returns `NullPtr` if either `rt` or `name` is null.
 */
enum AncorErrorCode ancora_tool_register(struct AncorRuntime *rt,
                                         const char *name,
                                         AncorToolCallback cb);

/**
 * Unregister a named tool callback. Returns `NullPtr` if either pointer is null.
 */
enum AncorErrorCode ancora_tool_unregister(struct AncorRuntime *rt, const char *name);

/**
 * Invoke a named tool with `input_bytes` and write the output into `out`.
 * Returns `NullPtr` if any required pointer is null, `Internal` if the tool is not found.
 */
enum AncorErrorCode ancora_tool_invoke(struct AncorRuntime *rt,
                                       const char *name,
                                       const uint8_t *input_bytes,
                                       uintptr_t input_len,
                                       struct AncorBuffer *out);

/**
 * Return the number of registered tools. Returns 0 if `rt` is null.
 */
uintptr_t ancora_tool_count(struct AncorRuntime *rt);

/**
 * Return 1 if a tool with `name` is registered, 0 otherwise. Returns 0 if any pointer is null.
 */
uint8_t ancora_tool_exists(struct AncorRuntime *rt, const char *name);

/**
 * Return the crate version as a null-terminated C string.
 */
const char *ancora_version(void);

#endif /* ANCORA_H */
