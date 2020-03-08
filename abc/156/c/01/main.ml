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

let square x = x * x

let n = scan "%d" identity
let xs = scan_list Int.of_string

let () =
  let half = (List.sum xs) / n in
  let half2 = (List.sum xs) / n + 1 in
  let n = List.sum @@ List.map (fun n -> square @@ n - half) xs in
  let m = List.sum @@ List.map (fun n -> square @@ n - half2) xs in
  printf "%d" @@ min n m
