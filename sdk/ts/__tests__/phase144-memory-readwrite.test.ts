class MemoryStore<T = unknown> {
  private store: Map<string, T> = new Map()
  write(key: string, value: T): void { this.store.set(key, value) }
  read(key: string, def?: T): T | undefined { return this.store.has(key) ? this.store.get(key) : def }
  delete(key: string): void { this.store.delete(key) }
  clear(): void { this.store.clear() }
  update(obj: Record<string, T>): void { Object.entries(obj).forEach(([k, v]) => this.store.set(k, v)) }
  pop(key: string, def?: T): T | undefined { const v = this.read(key, def); this.delete(key); return v }
  get keys(): string[] { return [...this.store.keys()] }
  get values(): T[] { return [...this.store.values()] }
}

describe('phase144 memory read write', () => {
  it('write and read returns same value', () => {
    const m = new MemoryStore<string>()
    m.write('k', 'v')
    expect(m.read('k')).toBe('v')
  })

  it('read missing key returns undefined', () => {
    const m = new MemoryStore()
    expect(m.read('nope')).toBeUndefined()
  })

  it('read missing key returns default', () => {
    const m = new MemoryStore<string>()
    expect(m.read('x', 'default')).toBe('default')
  })

  it('delete removes key', () => {
    const m = new MemoryStore<number>()
    m.write('x', 1)
    m.delete('x')
    expect(m.read('x')).toBeUndefined()
  })

  it('delete nonexistent is noop', () => {
    const m = new MemoryStore()
    expect(() => m.delete('ghost')).not.toThrow()
  })

  it('clear removes all keys', () => {
    const m = new MemoryStore<number>()
    m.write('a', 1)
    m.write('b', 2)
    m.clear()
    expect(m.read('a')).toBeUndefined()
    expect(m.read('b')).toBeUndefined()
  })

  it('update sets multiple keys', () => {
    const m = new MemoryStore<number>()
    m.update({ x: 10, y: 20 })
    expect(m.read('x')).toBe(10)
    expect(m.read('y')).toBe(20)
  })

  it('overwrite replaces value', () => {
    const m = new MemoryStore<string>()
    m.write('k', 'first')
    m.write('k', 'second')
    expect(m.read('k')).toBe('second')
  })

  it('keys property lists all keys', () => {
    const m = new MemoryStore<number>()
    m.write('one', 1)
    m.write('two', 2)
    expect(m.keys).toContain('one')
    expect(m.keys).toContain('two')
  })

  it('pop removes and returns value', () => {
    const m = new MemoryStore<string>()
    m.write('pop', 'val')
    expect(m.pop('pop')).toBe('val')
    expect(m.read('pop')).toBeUndefined()
  })
})
