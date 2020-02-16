open Printf

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let id a = a
let tup2 a b = (a, b)
let tup3 a b c = (a, b, c)

let (s, t) = scan "%s %s" tup2
let (a, b) = scan "%d %d" tup2
let u = scan "%s" id

let () =
  if u = s then
    printf "%d %d\n" (a - 1) b
  else
    printf "%d %d\n" a (b -1)
