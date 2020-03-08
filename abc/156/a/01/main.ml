open Batteries
open BatPrintf

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_list cnv =
  read_line ()
  |> String.split_on_char ' '
  |> List.map cnv

let scan_listn n cnv =
  (0 --^ n) |> Enum.map (fun _ -> (cnv % read_line) ())

let bsearch_ge arr a =
  let l = Array.length arr in
  match Array.bsearch Int.ord arr a with
  | `All_lower -> l
  | `All_bigger -> 0
  | `Just_after n -> l - n -1
  | `At n -> l - n
  | `Empty -> 0

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let (n, r) = scan "%d %d" Tuple.Tuple2.make

let () =
  printf "%d\n" (if n >= 10 then r
                 else r + 100*(10-n))
