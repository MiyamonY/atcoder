open Batteries
open Printf

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_list cnv =
  read_line ()
  |> BatString.split_on_char ' '
  |> List.map cnv

let id a = a
let tup2 a b = (a, b)
let tup3 a b c = (a, b, c)

let n = scan "%d" id
let an = scan_list int_of_string

let rec zip x y =
  match x with
  | [] -> []
  | a::xs ->
    match y with
    | [] -> []
    | b::ys -> (a, b) :: zip xs ys

let () =
  let l = List.sort compare an in
  zip l @@ List.tl l
  |> List.fold_left (fun b (x, y) -> if x = y then false else b ) true
  |> fun b -> printf @@ if b then "YES\n" else  "NO\n"
