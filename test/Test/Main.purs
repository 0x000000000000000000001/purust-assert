module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Console (log)
import Test.Assert (assert, assert', assertEqual, assertEqual', assertFalse, assertFalse', assertThrows, assertThrows', assertTrue, assertTrue')

foreign import scenario :: Int
foreign import throwing :: Unit -> Int
foreign import checkDeferredFailure :: (Unit -> Effect Unit) -> Effect Unit
foreign import checkDeferredThrows :: (Unit -> Effect Unit) -> Effect Unit

main :: Effect Unit
main = case scenario of
  0 -> do
    assert true
    assert' "custom success" true
    assertTrue true
    assertTrue' "custom true" true
    assertFalse false
    assertFalse' "custom false" false
    assertEqual { actual: 42, expected: 42 }
    assertEqual { actual: "same", expected: "same" }
    assertEqual' "custom equality" { actual: true, expected: true }
    assertThrows throwing
    assertThrows' "custom throws" throwing
    checkDeferredFailure (\_ -> assert' "deferred assertion" false)
    checkDeferredThrows (\_ -> assertThrows throwing)
    log "assert: positive cases and deferred replay passed"
  1 -> assert false
  2 -> assert' "custom assertion failure" false
  3 -> assertTrue false
  4 -> assertFalse true
  5 -> assertEqual { actual: 1, expected: 2 }
  6 -> assertEqual' "custom equality failure" { actual: "actual", expected: "expected" }
  7 -> assertThrows (\_ -> 7)
  8 -> assertThrows' "custom missing exception" (\_ -> unit)
  9 -> assertTrue' "custom true failure" false
  10 -> assertFalse' "custom false failure" true
  _ -> assert' "unknown scenario" false
