open Batteries

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_list cnv =
  read_line ()
  |> String.split_on_char ' '
  |> List.map cnv

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let s = scan "%s" identity

let () =
  let n = String.length s in
  let length i j =
    let sub = String.slice ~first:i ~last:j s in
    if List.for_all (String.contains "AGCT") @@ String.to_list sub then
      String.length sub else 0 in
  let rec aux f i j =
    if j > n then
      if i > n - 1 then []
      else
        aux f (i+1) (i+2)
    else
      (f i j) :: aux f i (j+1)
  in
  Printf.printf "%d\n" @@ List.max @@ aux length 0 1
