fun accumulate (f, []) = []
  | accumulate (f, x::xs) = (f x) :: (accumulate(f, xs))
