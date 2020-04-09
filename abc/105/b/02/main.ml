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

let n = scan "%d" identity

let dividens m =
  List.map (fun n -> m mod n = 0)@@ List.range 1 `To m
  |> List.filter identity
  |> List.length


let rec aux m =
  if m <= n then
    (if m mod 2 = 1 && dividens m = 8 then 1
     else 0)
    + aux (m+1)
  else
    0

let () =
  Printf.printf "%d\n" @@ aux 1
