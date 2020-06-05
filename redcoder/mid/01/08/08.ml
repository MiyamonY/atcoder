open Batteries

let scan fmt f =
  Scanf.sscanf (read_line ()) fmt f

let scan_lines n fmt f =
  List.Labels.map
    ~f:(fun _ -> scan fmt f)
    (List.range 1 `To n)

let scan_list cnv =
  read_line ()
  |> String.split_on_char ' '
  |> List.map cnv

let scan_matrix n m e conv =
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

let dbg x = Printf.eprintf "[debug]%s\n" @@ dump x; x

let n = scan "%d" identity
let ls = scan_lines n "%d %d" Tuple.Tuple2.make

let () =
  (List.Labels.map ls  ~f:(fun (a,_) ->
       List.Labels.map ls ~f:(fun (_,b) ->
           List.Labels.map ls ~f:(fun (a0, b0) -> (Int.abs (a-a0)) + b0-a0 + (Int.abs (b-b0)))
           |> List.sum)))
  |> List.concat
  |> List.min
  |> Printf.printf "%d\n"
