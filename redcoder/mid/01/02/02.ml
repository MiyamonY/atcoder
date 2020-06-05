open Batteries

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_list cnv =
  read_line ()
  |> String.split_on_char ' '
  |> List.map cnv

let scan_array n m e conv =
  let arr = Array.make_matrix n m e in
  Enum.Labels.iter (0 --^ n)
    ~f:(fun i -> arr.(i) <- Array.of_list @@ scan_list conv);
  arr

let rec zip xs ys =
  match xs, ys with
  | [], _ -> []
  | _, [] -> []
  | x::xs, y::ys ->
    (x, y) :: zip xs ys

let dbg x = Printf.printf "%s\n" @@ dump x; x

let n = scan "%d" identity

let aux n =
  Enum.Labels.map (1 -- n)
    ~f:(fun i -> if n mod i = 0 then 1 else 0 )
  |> Enum.sum
  |> (=) 8

let () =
  Enum.Labels.map (1 -- n)
    ~f:(fun i -> if i mod 2 = 1 && aux i then 1 else 0)
  |> Enum.sum
  |> Printf.printf "%d\n"
