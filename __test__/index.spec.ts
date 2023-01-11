import test from 'ava'

import { fetcher, plus100, plus101, plus100String } from '../index'

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
  t.is(plus101(fixture), fixture + 101)
  t.is(plus100String(fixture), fixture + 100 + '')
})

test('sync fetch', (t) => {
  t.is(fetcher('https://mockbin.org/status/200'), 'OK')
  t.is(fetcher('hps://badurl'), 'Error')
})
