import { describe, expect, test } from 'vitest'

describe('pass', () => {
  test('one', () => {
    expect(1).toBe(1)
  })

  test('true', () => {
    expect(true).toBe(true)
  })
})
