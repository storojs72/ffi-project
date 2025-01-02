module Main where

import BiniusFfi

main :: IO ()

-- TODO / FIXME
-- Currently program is fixed (it is a two u32 values addition) and defined
-- in binius-ffi/src/lib.rs. We pass actual values of that program to the 'prove' function.
--
-- Eventually we should use a whole program as an input to a 'prove' FFI function.
-- This program has to be represented as bytes, which are actually a serialized
-- system of constraints and witness defined by Binius.
--
-- The output of 'prove' FFI function is a serialized Binius proof. During verification, that might happen in
-- a different environment (for example on chain), the proof has to be deserialized and processed.

out = prove 25 25
main = putStrLn "Proving done"
