@[extern "lean_poseidon_hash_arity_4"] opaque blah : (a b c d : ByteArray) → ByteArray 

def main : IO Unit := do
  let a : ByteArray := .mk #[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
  IO.println $ blah a a a a