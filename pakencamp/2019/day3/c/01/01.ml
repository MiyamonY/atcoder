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

let (n, m) = scan "%d %d" @@ Tuple.Tuple2.make
let arr = scan_array n m 0 Int.of_string

let () =
  Enum.Labels.map (0 --^ m)
    ~f:(fun x -> Enum.Labels.map (0 --^ m)
           ~f:(fun y ->
               if x < y then
                 Enum.Labels.map (0 --^ n) ~f:(fun i -> max arr.(i).(x) arr.(i).(y)) |> Enum.sum
               else 0))
  |> Enum.concat
  |> Enum.reduce max
  |> Printf.printf "%d\n"
