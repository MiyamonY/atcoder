open Batteries
open Printf

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_list cnv =
  read_line ()
  |> BatString.split_on_char ' '
  |> List.map cnv

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let (n, k) = scan "%d %d" BatTuple.Tuple2.make
let ps = scan_list float_of_string

let exp n = n *. (n +. 1.0) /. 2.0 /. n

let hold_max (m, p) (x, y) =
  let n = p -. x +. y in
  (BatFloat.max m n, n)

let () =
  let sums = List.map exp ps in
  let init = BatList.fsum @@ BatList.take k sums in
  BatList.fold_left hold_max (init, init) (zip sums @@ BatList.drop k sums)
  |> BatTuple.Tuple2.first
  |> printf "%f\n"
